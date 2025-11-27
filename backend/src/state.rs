use std::sync::Arc;
use tokio::sync::broadcast;
use shared::MessageWrapper;
use dashmap::DashMap;

#[derive(Clone)]
pub struct AppState {
    // Broadcast channel for pushing updates to WebSockets
    pub tx: broadcast::Sender<MessageWrapper>,
    // Shared state for latest values (optional, for initial state on connection)
    pub latest_values: Arc<DashMap<u8, MessageWrapper>>,
    // Channel to send UDP packets (commands)
    pub udp_tx: tokio::sync::mpsc::Sender<Vec<u8>>,
}

impl AppState {
    pub fn new(udp_tx: tokio::sync::mpsc::Sender<Vec<u8>>) -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Self {
            tx,
            latest_values: Arc::new(DashMap::new()),
            udp_tx,
        }
    }
}
