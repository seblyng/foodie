use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    extract::{
        ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use common::websocket::FoodieMessageType;
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::{app::AppState, auth_backend::AuthSession, storage::FoodieStorage};

pub async fn websocket_handler<T>(
    ws: WebSocketUpgrade,
    auth: AuthSession,
    State(state): State<AppState<T>>,
) -> Response
where
    T: FoodieStorage + Send + Sync + Clone,
{
    let auth = auth.user.unwrap();
    ws.on_upgrade(move |socket| handle_socket(socket, auth.id, state.connections))
}

async fn handle_socket(
    mut socket: WebSocket,
    user_id: i32,
    connections: Arc<RwLock<HashMap<i32, UnboundedSender<FoodieMessageType>>>>,
) {
    let (tx, mut rx) = mpsc::unbounded_channel::<FoodieMessageType>();
    connections.write().unwrap().insert(user_id, tx);

    while let Some(msg) = rx.recv().await {
        let bytes = Utf8Bytes::from(serde_json::to_string(&msg).unwrap());
        if socket.send(Message::Text(bytes)).await.is_err() {
            break;
        }
    }

    connections.write().unwrap().remove(&user_id);
}
