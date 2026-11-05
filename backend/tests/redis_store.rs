use std::{collections::HashMap, time::Duration};

use backend::redis_store::RedisStore;
use time::OffsetDateTime;
use tower_sessions_core::{session::Record, SessionStore};

static REDIS_TEST_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

#[tokio::test]
#[ignore = "Requires a dedicated Redis instance: this test disconnects its clients"]
async fn sessions_recover_after_connection_loss() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url =
        std::env::var("REDIS_TEST_URL").expect("Set REDIS_TEST_URL to a dedicated test server");
    let store = RedisStore::new(url.clone()).await.unwrap();
    let mut record = Record {
        id: Default::default(),
        data: HashMap::from([("probe".to_owned(), serde_json::json!("before"))]),
        expiry_date: OffsetDateTime::now_utc() + time::Duration::minutes(5),
    };
    store.create(&mut record).await.unwrap();
    assert!(store.load(&record.id).await.unwrap().is_some());

    let mut admin = redis::Client::open(url)
        .unwrap()
        .get_multiplexed_async_connection()
        .await
        .unwrap();
    let disconnected: usize = redis::cmd("CLIENT")
        .arg("KILL")
        .arg("TYPE")
        .arg("normal")
        .arg("SKIPME")
        .arg("yes")
        .query_async(&mut admin)
        .await
        .unwrap();
    assert!(disconnected >= 1);

    let restored = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            if let Ok(Some(record)) = store.load(&record.id).await {
                break record;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("Session store did not reconnect");
    assert_eq!(restored.data, record.data);

    record
        .data
        .insert("probe".to_owned(), serde_json::json!("after"));
    store.save(&record).await.unwrap();
    assert_eq!(
        store.load(&record.id).await.unwrap().unwrap().data,
        record.data
    );
    store.delete(&record.id).await.unwrap();
    assert!(store.load(&record.id).await.unwrap().is_none());
}

#[tokio::test]
#[ignore = "Requires a dedicated unauthenticated Redis instance"]
async fn unresponsive_commands_have_a_bounded_timeout() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    use tokio::{
        io::copy_bidirectional,
        net::{TcpListener, TcpStream},
        sync::{oneshot, watch},
    };

    let url =
        std::env::var("REDIS_TEST_URL").expect("Set REDIS_TEST_URL to a dedicated test server");
    let client = redis::Client::open(url).unwrap();
    let (host, port) = match &client.get_connection_info().addr {
        redis::ConnectionAddr::Tcp(host, port) => (host.clone(), *port),
        _ => panic!("This regression test requires plain TCP Redis"),
    };
    assert!(client.get_connection_info().redis.password.is_none());
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let proxy_port = listener.local_addr().unwrap().port();
    let (stall, stalled) = watch::channel(false);
    let (ack, acknowledged) = oneshot::channel();
    let proxy = tokio::spawn(async move {
        let mut first = Some(ack);
        loop {
            let (mut downstream, _) = listener.accept().await.unwrap();
            let mut upstream = TcpStream::connect((host.as_str(), port)).await.unwrap();
            let mut stalled = stalled.clone();
            let ack = first.take();
            tokio::spawn(async move {
                if let Some(ack) = ack {
                    tokio::select! {
                        _ = copy_bidirectional(&mut downstream, &mut upstream) => return,
                        _ = stalled.changed() => {},
                    }
                    let _ = ack.send(());
                    // Retain the sockets but stop forwarding Redis responses.
                    std::future::pending::<()>().await;
                    drop((downstream, upstream));
                } else {
                    let _ = copy_bidirectional(&mut downstream, &mut upstream).await;
                }
            });
        }
    });

    let store = RedisStore::new(format!("redis://127.0.0.1:{proxy_port}/"))
        .await
        .unwrap();
    let mut record = Record {
        id: Default::default(),
        data: HashMap::from([("probe".to_owned(), serde_json::json!("preserved"))]),
        expiry_date: OffsetDateTime::now_utc() + time::Duration::minutes(5),
    };
    store.create(&mut record).await.unwrap();
    stall.send(true).unwrap();
    acknowledged.await.unwrap();
    let result = tokio::time::timeout(Duration::from_secs(5), store.load(&record.id))
        .await
        .expect("Session command remained stuck on an unresponsive connection");
    assert!(result.is_err());
    let mut admin = client.get_multiplexed_async_connection().await.unwrap();
    let _: usize = redis::cmd("DEL")
        .arg(record.id.to_string())
        .query_async(&mut admin)
        .await
        .unwrap();
    proxy.abort();
}
