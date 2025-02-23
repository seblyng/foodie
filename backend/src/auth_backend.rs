use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use common::user::User;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    EndpointNotSet, EndpointSet, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest::{Client, Url};
use sea_orm::{sea_query::OnConflict, ActiveValue::NotSet, DatabaseConnection, EntityTrait, Set};
use serde::Deserialize;

use crate::entities::{self, users};

pub type Oauth2ClientWithEndpoints =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

#[derive(Clone)]
pub struct Backend {
    db: DatabaseConnection,
    client: Oauth2ClientWithEndpoints,
}

impl Backend {
    pub fn new(db: DatabaseConnection, client: Oauth2ClientWithEndpoints) -> Self {
        Self { db, client }
    }

    pub fn authorize_url(&self) -> (Url, CsrfToken) {
        self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.profile".to_string(),
            ))
            .url()
    }
}

#[derive(Clone)]
pub struct Credentials {
    code: String,
}

impl Credentials {
    pub fn new(code: String) -> Self {
        Self { code }
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct GoogleUserResult {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let client = Client::new();
        let token = self
            .client
            .exchange_code(AuthorizationCode::new(credentials.code))
            .request_async(&client)
            .await
            .unwrap();

        let url = Url::parse("https://www.googleapis.com/oauth2/v1/userinfo").unwrap();
        let response = client
            .get(url)
            .bearer_auth(token.access_token().secret())
            .send()
            .await
            .unwrap();

        let google_user = response.json::<GoogleUserResult>().await.unwrap();

        // TODO: Do not hardcode access to login/create user
        if google_user.email != "sebastian@lyngjohansen.com" {
            return Ok(None);
        }

        let user_model = users::Entity::insert(crate::entities::users::ActiveModel {
            id: NotSet,
            password: NotSet,
            name: Set(google_user.name),
            email: Set(google_user.email),
        })
        .on_conflict(
            OnConflict::column(users::Column::Email)
                .update_column(users::Column::Email)
                .to_owned(),
        )
        .exec_with_returning(&self.db)
        .await
        .unwrap();

        Ok(Some(User {
            id: user_model.id,
            name: user_model.email,
            email: user_model.name,
        }))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = entities::users::Entity::find_by_id(*user_id)
            .one(&self.db)
            .await
            .unwrap()
            .unwrap();

        Ok(Some(User {
            id: user.id,
            name: user.name,
            email: user.email,
        }))
    }
}

pub fn get_oauth_client() -> Result<Oauth2ClientWithEndpoints, anyhow::Error> {
    let base_url = dotenv::var("BASE_URL")?;
    let client_id = dotenv::var("GOOGLE_CLIENT_ID")?;
    let client_secret = dotenv::var("GOOGLE_CLIENT_SECRET")?;
    let redirect_url = format!("{}/api/oauth/google/callback", base_url);

    // access_type=offline&prompt=consent makes it return a refresh token
    let auth_url = "https://accounts.google.com/o/oauth2/auth".to_string();
    let token_url = "https://accounts.google.com/o/oauth2/token".to_string();

    let basic_client: Oauth2ClientWithEndpoints = BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(AuthUrl::new(auth_url)?)
        .set_token_uri(TokenUrl::new(token_url)?)
        .set_redirect_uri(RedirectUrl::new(redirect_url)?);

    Ok(basic_client)
}

pub type AuthSession = axum_login::AuthSession<Backend>;
