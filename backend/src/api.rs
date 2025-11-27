use crate::state::AppState;
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};

use tracing::{error, info};

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

    // Send latest values to the new client
    for entry in state.latest_values.iter() {
        let msg = entry.value();
        if let Ok(bytes) = msg.to_bytes() {
             if let Err(e) = sender.send(Message::Binary(bytes)).await {
                error!("Error sending initial state: {}", e);
                return;
            }
        }
    }

    // Spawn a task to forward broadcast messages to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // Serialize message to JSON for WebSocket
            // Note: In a real high-perf scenario, we might want to use binary (Protobuf) over WS too.
            // But for web frontend ease, JSON is often preferred unless performance is critical.
            // Given the requirements mention "Receive packets and deserialize them using shared types (Protobuf)",
            // but doesn't explicitly say WS must be Protobuf. Let's stick to JSON for now as it's easier for Leptos/JS.
            // Wait, shared types are Protobuf generated. They might not implement Serialize/Deserialize by default unless configured.
            // Let's check if `shared` crate enables serde for prost types.
            // The `prost-build` config in `shared/build.rs` (which I can't see but assume) or `prost` attributes need to be set.
            // If not, we might need to send binary.
            // Let's assume binary for now to be safe and consistent with "bridge".
            
            match msg.to_bytes() {
                Ok(bytes) => {
                    if let Err(e) = sender.send(Message::Binary(bytes)).await {
                        error!("Error sending WS message: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    error!("Error serializing message: {}", e);
                }
            }
        }
    });

    // Handle incoming messages from this client
    let udp_tx = state.udp_tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Binary(bytes) => {
                    // Forward binary messages directly to UDP
                    // We assume the frontend sends valid MessageWrapper bytes
                    if let Err(e) = udp_tx.send(bytes).await {
                        error!("Error forwarding WS message to UDP: {}", e);
                        break;
                    }
                }
                Message::Text(_) => {
                    // Ignore text messages for now, or handle specific commands
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
    
    info!("WebSocket connection closed");
}
