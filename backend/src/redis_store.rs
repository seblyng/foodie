use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use redis::{
    aio::{ConnectionManager, ConnectionManagerConfig},
    AsyncCommands, ExistenceCheck, SetExpiry,
};
use time::OffsetDateTime;
use tower_sessions_core::{
    session::{Id, Record},
    session_store, SessionStore,
};

/// A Redis session store.
#[derive(Clone)]
pub struct RedisStore {
    conn: ConnectionManager,
}

impl Debug for RedisStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisStore").finish_non_exhaustive()
    }
}

impl RedisStore {
    pub async fn new(connection_string: String) -> Result<Self, anyhow::Error> {
        // Pod replacement can leave an established TCP flow black-holed rather
        // than closed. Bound both commands and reconnect attempts so the
        // manager detects that failure and establishes a fresh connection.
        let tcp_settings = redis::io::tcp::TcpSettings::default().set_keepalive(
            redis::io::tcp::socket2::TcpKeepalive::new()
                .with_time(Duration::from_secs(3))
                .with_interval(Duration::from_secs(1)),
        );
        #[cfg(target_os = "linux")]
        let tcp_settings = tcp_settings.set_user_timeout(Duration::from_secs(3));
        let config = ConnectionManagerConfig::new()
            .set_response_timeout(Duration::from_secs(3))
            .set_connection_timeout(Duration::from_secs(3))
            .set_max_delay(1000)
            .set_tcp_settings(tcp_settings);
        let conn = redis::Client::open(connection_string)?
            .get_connection_manager_with_config(config)
            .await?;

        Ok(Self { conn })
    }

    async fn save_with_options(
        &self,
        record: &Record,
        existence: ExistenceCheck,
    ) -> session_store::Result<bool> {
        let val = serde_json::to_string(&record)
            .map_err(|err| session_store::Error::Encode(err.to_string()))?;

        let opts = redis::SetOptions::default()
            .conditional_set(existence)
            .get(false)
            .with_expiration(SetExpiry::EX(
                OffsetDateTime::unix_timestamp(record.expiry_date) as u64,
            ));

        let res: bool = self
            .conn
            .clone()
            .set_options(record.id.to_string(), val, opts)
            .await
            .map_err(|err| session_store::Error::Backend(err.to_string()))?;

        Ok(res)
    }
}

#[async_trait]
impl SessionStore for RedisStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        while !self.save_with_options(record, ExistenceCheck::NX).await? {
            record.id = Id::default();
        }
        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        self.save_with_options(record, ExistenceCheck::XX).await?;
        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let id = session_id.to_string();
        let data = self
            .conn
            .clone()
            .get::<String, Option<String>>(id)
            .await
            .map_err(|err| session_store::Error::Backend(err.to_string()))?;

        if let Some(data) = data {
            let record = serde_json::from_str(&data)
                .map_err(|err| session_store::Error::Decode(err.to_string()))?;
            Ok(Some(record))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let _: () = self
            .conn
            .clone()
            .del(session_id.to_string())
            .await
            .map_err(|err| session_store::Error::Backend(err.to_string()))?;
        Ok(())
    }
}
