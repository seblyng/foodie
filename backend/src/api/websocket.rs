use axum::{
    extract::{
        ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use common::websocket::FoodieMessageType;
use tokio::sync::broadcast;

use crate::{app::AppState, storage::FoodieStorage};

pub async fn websocket_handler<T>(
    ws: WebSocketUpgrade,
    State(state): State<AppState<T>>,
) -> Response
where
    T: FoodieStorage + Send + Sync + Clone,
{
    ws.on_upgrade(move |socket| handle_socket(socket, state.tx))
}

async fn handle_socket(mut socket: WebSocket, tx: broadcast::Sender<FoodieMessageType>) {
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    let bytes = Utf8Bytes::from(serde_json::to_string(&msg).unwrap());
                    if socket.send(Message::Text(bytes)).await.is_err() {
                        break;
                    }
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {}
                Err(_) => break,
            }
        }
    });
}
