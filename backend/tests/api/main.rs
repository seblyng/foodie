mod recipe;

use axum::{error_handling::HandleErrorLayer, routing::post, Router};
use axum_login::AuthManagerLayerBuilder;
use common::user::User;
use hyper::StatusCode;
use migration::MigratorTrait;
use sea_orm::{ActiveValue::NotSet, DatabaseConnection, EntityTrait, Set, SqlxPostgresConnector};
use serde::Serialize;
use sqlx::PgPool;
use std::{fmt::Display, future::IntoFuture};
use tower::ServiceBuilder;

use backend::{app::App, auth_backend::AuthSession, entities::users};
use reqwest::{IntoUrl, RequestBuilder, Response};

struct TestApp {
    pub client: reqwest::Client,
    pub address: String,
    pool: DatabaseConnection,
}

const TEST_EMAIL: &str = "foo@foo.com";
const TEST_NAME: &str = "foo";

// TODO: Move this to another place to make the implementation details hidden from the tests.
// Now the tests are able to access non public fields and methods which I do not like, as they are
// in the same module

// Maybe I want different `post_unauth` or something similar for the endpoints that should be
// open. I am not sure if this is really needed, but think about it.
impl TestApp {
    async fn new(pool: PgPool) -> Result<Self, anyhow::Error> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind to port");

        let address = format!("http://{}", listener.local_addr()?);
        let connection = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);

        let app = App::new(connection.clone()).await?;

        let auth_service = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|_| async {
                StatusCode::UNAUTHORIZED
            }))
            .layer(AuthManagerLayerBuilder::new(app.backend, app.session_layer).build());

        let router = app
            .router
            .merge(
                Router::new()
                    .route("/test-login", post(login))
                    .route("/test-logout", post(logout)),
            )
            .layer(auth_service);

        let server = axum::serve(listener, router.into_make_service());
        tokio::spawn(server.into_future());

        migration::Migrator::up(&connection, None).await?;

        users::Entity::insert(users::ActiveModel {
            id: NotSet,
            email: Set(TEST_EMAIL.to_string()),
            password: NotSet,
            name: Set(TEST_NAME.to_string()),
        })
        .exec(&app.app_state.db)
        .await?;

        let client = reqwest::Client::builder().cookie_store(true).build()?;

        Ok(Self {
            address,
            client,
            pool: connection,
        })
    }

    pub async fn post<T, U>(&self, url: U, body: &T) -> Result<Response, anyhow::Error>
    where
        U: IntoUrl + Display,
        T: Serialize + ?Sized,
    {
        let url = format!("{}/{}", self.address, url);
        let req = self.client.request(reqwest::Method::POST, &url).json(body);

        self.assert_logged_in(req.try_clone().unwrap(), &url).await;

        self.login().await;

        let res = req.send().await?;

        self.logout().await;

        Ok(res)
    }

    pub async fn put<T, U>(&self, url: U, body: &T) -> Result<Response, anyhow::Error>
    where
        U: IntoUrl + Display,
        T: Serialize + ?Sized,
    {
        let url = format!("{}/{}", self.address, url);
        let req = self.client.request(reqwest::Method::PUT, &url).json(body);

        self.assert_logged_in(req.try_clone().unwrap(), &url).await;

        self.login().await;

        let res = req.send().await?;

        self.logout().await;

        Ok(res)
    }

    pub async fn delete<U: IntoUrl + Display>(&self, url: U) -> Result<Response, anyhow::Error> {
        let url = format!("{}/{}", self.address, url);
        let req = self.client.request(reqwest::Method::DELETE, &url);

        self.assert_logged_in(req.try_clone().unwrap(), &url).await;

        self.login().await;

        let res = req.send().await?;

        self.logout().await;

        Ok(res)
    }

    pub async fn get<U: IntoUrl + Display>(&self, url: U) -> Result<Response, anyhow::Error> {
        let url = format!("{}/{}", self.address, url);
        let req = self.client.request(reqwest::Method::GET, &url);

        self.assert_logged_in(req.try_clone().unwrap(), &url).await;

        self.login().await;

        let res = req.send().await?;

        self.logout().await;

        Ok(res)
    }

    async fn login(&self) {
        self.client
            .request(
                reqwest::Method::POST,
                format!("{}/test-login", self.address),
            )
            .send()
            .await
            .unwrap();
    }

    async fn logout(&self) {
        self.client
            .request(
                reqwest::Method::POST,
                format!("{}/test-logout", self.address),
            )
            .send()
            .await
            .unwrap();
    }

    async fn assert_logged_in(&self, req: RequestBuilder, url: &str) {
        let response = req.send().await.unwrap();

        assert_eq!(
            response.status(),
            reqwest::StatusCode::UNAUTHORIZED,
            "{} is not protected",
            url
        );
    }
}

async fn login(mut auth: AuthSession) {
    auth.login(&User {
        id: 1,
        name: TEST_NAME.to_string(),
        email: TEST_EMAIL.to_string(),
    })
    .await
    .unwrap();
}

async fn logout(mut auth: AuthSession) {
    auth.logout().await.unwrap();
}
