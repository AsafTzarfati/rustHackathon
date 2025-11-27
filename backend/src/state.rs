use std::sync::Arc;
use tokio::sync::broadcast;
use dashmap::DashMap;
use shared::proto::{SensorBatch, SystemStatus, HardwareStatus};
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct AppState {
    pub sensor_data: Arc<DashMap<String, SensorBatch>>, // Keyed by source or some ID
    pub system_status: Arc<DashMap<String, SystemStatus>>,
    pub hardware_status: Arc<DashMap<String, HardwareStatus>>,
    pub tx: broadcast::Sender<BroadcastMessage>,
    pub udp_tx: tokio::sync::mpsc::Sender<Vec<u8>>,
}

impl AppState {
    pub fn new(udp_tx: tokio::sync::mpsc::Sender<Vec<u8>>) -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            sensor_data: Arc::new(DashMap::new()),
            system_status: Arc::new(DashMap::new()),
            hardware_status: Arc::new(DashMap::new()),
            tx,
            udp_tx,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum BroadcastMessage {
    SensorUpdate(SensorBatch),
    SystemStatusUpdate(SystemStatus),
    HardwareStatusUpdate(HardwareStatus),
}
