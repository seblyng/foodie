use std::{collections::HashMap, time::Duration};

use backend::redis_store::RedisStore;
use time::OffsetDateTime;
use tower_sessions_core::{session::Record, SessionStore};

#[tokio::test]
#[ignore = "Requires a dedicated Redis instance: this test disconnects its clients"]
async fn sessions_recover_after_connection_loss() {
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
