use crate::{
    api::{
        auth::{get_me, login, logout, register},
        friends::{accept_friendship, get_pending, reject_friendship, send_friend_request},
        ingredient::{delete_ingredient, get_ingredient, get_ingredients, post_ingredient},
        oauth::{google_callback, google_login},
        recipe::{
            delete_recipe, get_presigned_url_for_upload, get_recipe, get_recipes, post_recipe,
            update_recipe,
        },
        users::get_users,
        websocket::websocket_handler,
    },
    auth_backend::{get_oauth_client, Backend},
    storage::{self, FoodieStorage},
};
use axum::{
    error_handling::HandleErrorLayer,
    extract::FromRef,
    http::{HeaderValue, StatusCode},
    routing::{any, get, post},
    Router,
};
use axum_login::{
    login_required,
    tower_sessions::{cookie::time::Duration, Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use common::websocket::FoodieMessageType;
use hyper::{header::CONTENT_TYPE, Method};
use sea_orm::DatabaseConnection;
use std::{
    collections::HashMap,
    sync::{Arc, Once, RwLock},
};
use tokio::sync::mpsc::UnboundedSender;
use tower::ServiceBuilder;
use tower_http::{catch_panic::CatchPanicLayer, cors::CorsLayer};
use tower_sessions_core::SessionStore;

#[derive(Clone)]
pub struct AppState<T>
where
    T: FoodieStorage + Clone + Sync + Send,
{
    pub db: DatabaseConnection,
    pub storage: T,
    pub connections: Arc<RwLock<HashMap<i32, UnboundedSender<FoodieMessageType>>>>,
}

impl<T> FromRef<AppState<T>> for DatabaseConnection
where
    T: FoodieStorage + Clone + Sync + Send,
{
    fn from_ref(input: &AppState<T>) -> Self {
        input.db.clone()
    }
}

pub struct App {
    pub router: Router,
}

static INIT: Once = Once::new();

impl App {
    pub async fn new<S>(db: DatabaseConnection, session_store: S) -> Result<Self, anyhow::Error>
    where
        S: SessionStore + Clone,
    {
        INIT.call_once(|| {
            env_logger::builder()
                .is_test(true)
                .format_timestamp(None)
                .filter_level(log::LevelFilter::Error)
                .init();
        });

        let oauth_client = get_oauth_client()?;

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(!cfg!(debug_assertions))
            .with_expiry(Expiry::OnInactivity(Duration::days(1)));

        let backend = Backend::new(db.clone(), oauth_client);
        let auth_service = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|_| async {
                StatusCode::UNAUTHORIZED
            }))
            .layer(AuthManagerLayerBuilder::new(backend, session_layer).build());

        let storage = storage::aws::FoodieAws::new().await;
        let connections = Arc::new(RwLock::new(HashMap::new()));
        let app_state = AppState {
            db,
            storage,
            connections,
        };

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT])
            .allow_credentials(true)
            .allow_headers([CONTENT_TYPE])
            .allow_origin("http://localhost:8080".parse::<HeaderValue>()?);

        let router = Router::new()
            .nest(
                "/api",
                Router::new()
                    .nest(
                        "/friends",
                        Router::new()
                            .route("/new/{id}", post(send_friend_request))
                            .route("/accept/{id}", post(accept_friendship))
                            .route("/reject/{id}", post(reject_friendship))
                            .route("/pending", get(get_pending)),
                    )
                    .nest(
                        "/recipes",
                        Router::new()
                            .route("/", get(get_recipes).post(post_recipe))
                            .route(
                                "/{id}",
                                get(get_recipe).delete(delete_recipe).put(update_recipe),
                            ),
                    )
                    .nest(
                        "/ingredients",
                        Router::new()
                            .route("/ingredients", post(post_ingredient).get(get_ingredients))
                            .route(
                                "/ingredients/{id}",
                                get(get_ingredient).delete(delete_ingredient),
                            ),
                    )
                    .route("/uploads/recipes/images", get(get_presigned_url_for_upload))
                    .route("/users", get(get_users))
                    .route("/me", get(get_me))
                    .route("/ws", any(websocket_handler))
                    .route_layer(login_required!(Backend))
                    .route("/health-check", get(|| async {}))
                    .route("/register", post(register))
                    .route("/login", post(login))
                    .route("/logout", post(logout))
                    .nest(
                        "/oauth",
                        Router::new()
                            .route("/google/login", get(google_login))
                            .route("/google/callback", get(google_callback)),
                    ),
            )
            .with_state(app_state)
            .layer(auth_service)
            .layer(CatchPanicLayer::new())
            .layer(cors);

        Ok(Self { router })
    }
}
