use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use crate::state::AppState;
use futures::{sink::SinkExt, stream::StreamExt};
use tracing::{info, error};

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    // Spawn a task to forward broadcast messages to the websocket
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Handle incoming messages from the websocket (commands)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                info!("Received from WS: {}", text);
                // Try to deserialize to ActuatorCommand
                if let Ok(cmd) = serde_json::from_str::<shared::proto::ActuatorCommand>(&text) {
                     use prost::Message;
                     let mut buf = Vec::new();
                     if let Ok(_) = cmd.encode(&mut buf) {
                         let _ = state.udp_tx.send(buf).await;
                         info!("Sent ActuatorCommand via UDP");
                     }
                } else {
                    tracing::warn!("Failed to parse WS message as ActuatorCommand");
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
