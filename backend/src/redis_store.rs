use std::fmt::Debug;

use async_trait::async_trait;
use redis::{aio::MultiplexedConnection, AsyncCommands, ExistenceCheck, SetExpiry};
use time::OffsetDateTime;
use tower_sessions_core::{
    session::{Id, Record},
    session_store, SessionStore,
};

/// A Redis session store.
#[derive(Debug, Clone)]
pub struct RedisStore {
    conn: MultiplexedConnection,
}

impl RedisStore {
    pub async fn new(connection_string: String) -> Result<Self, anyhow::Error> {
        let conn = redis::Client::open(connection_string)?
            .get_multiplexed_async_connection()
            .await?;

        Ok(Self { conn })
    }

    async fn save_with_options(
        &self,
        record: &Record,
        existence: ExistenceCheck,
    ) -> session_store::Result<bool> {
        let val = serde_json::to_string(&record).unwrap();

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
            .unwrap();

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
            .unwrap();

        if let Some(data) = data {
            Ok(Some(serde_json::from_str(&data).unwrap()))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let _: () = self.conn.clone().del(session_id.to_string()).await.unwrap();
        Ok(())
    }
}
