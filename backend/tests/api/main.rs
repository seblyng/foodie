mod friends;
mod recipe;
mod users;

use common::user::{CreateUser, User, UserLogin};
use migration::MigratorTrait;
use sea_orm::{DatabaseConnection, SqlxPostgresConnector};
use serde::Serialize;
use sqlx::PgPool;
use std::{fmt::Display, future::IntoFuture};

use backend::app::App;
use reqwest::{IntoUrl, Response};

struct TestApp {
    pub client: reqwest::Client,
    pub address: String,
    pub user: User,
    pool: DatabaseConnection,
}

const TEST_EMAIL: &str = "foo@foo.com";
const TEST_NAME: &str = "foo";
const TEST_PASSWORD: &str = "foo";

impl TestApp {
    async fn new(pool: PgPool) -> Result<Self, anyhow::Error> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind to port");

        let address = format!("http://{}", listener.local_addr()?);
        let connection = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);

        let app = App::new(connection.clone()).await?;
        let server = axum::serve(listener, app.router.into_make_service());
        tokio::spawn(server.into_future());

        migration::Migrator::up(&connection, None).await?;

        let client = reqwest::Client::builder().cookie_store(true).build()?;
        // HACK: Set an env variable that I read in `register` API.
        // Not opening up so that anyone can just register a user
        std::env::set_var("FOODIE_TEST", "1");

        let user = client
            .post(format!("{}/api/register", address))
            .json(&CreateUser {
                name: TEST_NAME.to_string(),
                email: TEST_EMAIL.to_string(),
                password: TEST_PASSWORD.to_string(),
            })
            .send()
            .await?
            .json::<User>()
            .await?;

        let _self = Self {
            address,
            client,
            user,
            pool: connection,
        };

        _self
            .login(&UserLogin {
                email: TEST_EMAIL.to_string(),
                password: TEST_PASSWORD.to_string(),
            })
            .await;

        Ok(_self)
    }

    pub async fn post<T, U>(&self, url: U, body: Option<&T>) -> Result<Response, anyhow::Error>
    where
        U: IntoUrl + Display,
        T: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.address, url);
        let req = match body {
            Some(body) => self.client.request(reqwest::Method::POST, &url).json(body),
            None => self.client.request(reqwest::Method::POST, &url),
        };

        let res = req.send().await?;

        Ok(res)
    }

    pub async fn put<T, U>(&self, url: U, body: &T) -> Result<Response, anyhow::Error>
    where
        U: IntoUrl + Display,
        T: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.address, url);
        let req = self.client.request(reqwest::Method::PUT, &url).json(body);

        let res = req.send().await?;

        Ok(res)
    }

    pub async fn delete<U: IntoUrl + Display>(&self, url: U) -> Result<Response, anyhow::Error> {
        let url = format!("{}{}", self.address, url);
        let req = self.client.request(reqwest::Method::DELETE, &url);

        let res = req.send().await?;

        Ok(res)
    }

    pub async fn get<U: IntoUrl + Display>(&self, url: U) -> Result<Response, anyhow::Error> {
        let url = format!("{}{}", self.address, url);
        let req = self.client.request(reqwest::Method::GET, &url);

        let res = req.send().await?;

        Ok(res)
    }

    async fn login(&self, input: &UserLogin) {
        self.post("/api/login", Some(input)).await.unwrap();
    }

    async fn create_user(&self, input: &CreateUser) -> Result<User, anyhow::Error> {
        let user = self
            .post("/api/register", Some(input))
            .await?
            .json::<User>()
            .await?;
        Ok(user)
    }
}
