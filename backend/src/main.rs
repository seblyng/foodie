use backend::{app::App, redis_store::RedisStore};
use sea_orm::{ConnectOptions, Database};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = ConnectOptions::new(dotenv::var("DATABASE_URL")?);
    let db = Database::connect(opt).await?;
    let session_store = RedisStore::new(dotenv::var("REDIS_URL")?).await?;
    let app = App::new(db, session_store).await?;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069")
        .await
        .expect("Failed to bind to port");
    println!("Server running on {}", listener.local_addr()?);
    let server = axum::serve(listener, app.router.into_make_service());
    Ok(server.await?)
}
