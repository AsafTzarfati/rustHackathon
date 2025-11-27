use std::sync::Arc;
use tokio::net::UdpSocket;
use tracing::{info, error, warn};
use prost::Message;
use crate::state::{AppState, BroadcastMessage};
use shared::proto::{SensorBatch, SystemStatus, HardwareStatus};

pub async fn udp_listener(state: AppState, port: u16) -> std::io::Result<()> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).await?;
    info!("UDP Listener bound to 0.0.0.0:{}", port);

    let mut buf = [0u8; 65535]; // Max UDP size

    loop {
        match socket.recv_from(&mut buf).await {
            Ok((size, _addr)) => {
                let data = &buf[..size];
                // Heuristic decoding: Try most common messages
                // In a real system, we'd use a header or a wrapper message.
                
                if let Ok(msg) = SensorBatch::decode(data) {
                    // Update state and broadcast
                    // Assuming header.source is the key
                    let source = msg.header.as_ref().map(|h| h.source.clone()).unwrap_or_default();
                    state.sensor_data.insert(source, msg.clone());
                    let _ = state.tx.send(BroadcastMessage::SensorUpdate(msg));
                    continue;
                }

                if let Ok(msg) = SystemStatus::decode(data) {
                    let source = msg.header.as_ref().map(|h| h.source.clone()).unwrap_or_default();
                    state.system_status.insert(source, msg.clone());
                    let _ = state.tx.send(BroadcastMessage::SystemStatusUpdate(msg));
                    continue;
                }
                
                if let Ok(msg) = HardwareStatus::decode(data) {
                     let source = msg.header.as_ref().map(|h| h.source.clone()).unwrap_or_default();
                    state.hardware_status.insert(source, msg.clone());
                    let _ = state.tx.send(BroadcastMessage::HardwareStatusUpdate(msg));
                    continue;
                }

                warn!("Received UDP packet of size {} but could not decode as known message", size);
            }
            Err(e) => {
                error!("UDP receive error: {}", e);
            }
        }
    }
}

pub async fn udp_sender(target_addr: String, mut rx: tokio::sync::mpsc::Receiver<Vec<u8>>) {
    let socket = UdpSocket::bind("0.0.0.0:0").await.expect("Failed to bind sender socket");
    
    while let Some(data) = rx.recv().await {
        if let Err(e) = socket.send_to(&data, &target_addr).await {
            error!("Failed to send UDP packet to {}: {}", target_addr, e);
        }
    }
}
