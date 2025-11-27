mod api;
mod state;
mod udp;

use crate::state::AppState;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Channel for outbound UDP
    let (udp_tx, udp_rx) = tokio::sync::mpsc::channel(100);

    let state = AppState::new(udp_tx);
    
    // Start UDP Listener
    let udp_state = state.clone();
    tokio::spawn(async move {
        if let Err(e) = udp::udp_listener(udp_state, 5000).await {
            tracing::error!("UDP listener failed: {}", e);
        }
    });

    // Start UDP Sender
    let target_addr = std::env::var("REALTIME_HOST").unwrap_or_else(|_| "127.0.0.1:5001".to_string());
    tokio::spawn(async move {
        udp::udp_sender(target_addr, udp_rx).await;
    });

    // Start Axum Server
    let app = api::app_router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
