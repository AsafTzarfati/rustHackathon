use crate::state::AppState;
use shared::MessageWrapper;

use tokio::net::UdpSocket;
use tracing::{error, info, warn};

pub async fn udp_listener(state: AppState, port: u16) -> std::io::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let socket = UdpSocket::bind(&addr).await?;
    info!("UDP Listener started on {}", addr);

    let mut buf = [0u8; 65535]; // Max UDP size

    loop {
        match socket.recv_from(&mut buf).await {
            Ok((size, src)) => {
                let data = &buf[..size];
                match MessageWrapper::from_bytes(data) {
                    Ok(msg) => {
                        // Update latest values
                        // We use a simple mapping based on the message type
                        // Since we don't have a direct get_id, we can just store it.
                        // For this hackathon, let's just broadcast. 
                        // To fix the warning about unused `latest_values`, let's actually use it.
                        // We need to map the message to an ID.
                        let id = match &msg {
                            MessageWrapper::SensorBatch(_) => 1,
                            MessageWrapper::SystemStatus(_) => 2,
                            MessageWrapper::HardwareStatus(_) => 3,
                            MessageWrapper::ClockModulation(_) => 4,
                            MessageWrapper::TestCase(_) => 5,
                            MessageWrapper::SimulationState(_) => 6,
                            MessageWrapper::TestResult(_) => 7,
                            MessageWrapper::TimeSync(_) => 8,
                            MessageWrapper::FaultInjection(_) => 9,
                            MessageWrapper::ActuatorCommand(_) => 10,
                            MessageWrapper::Heartbeat(_) => 11,
                            MessageWrapper::Ack(_) => 12,
                        };
                        state.latest_values.insert(id, msg.clone());

                        // Broadcast to WebSockets
                        if let Err(_e) = state.tx.send(msg.clone()) {
                            // It's okay if no one is listening
                            // warn!("Failed to broadcast message: {}", e);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to deserialize packet from {}: {}", src, e);
                    }
                }
            }
            Err(e) => {
                error!("UDP receive error: {}", e);
            }
        }
    }
}

pub async fn udp_sender(target_addr: String, mut rx: tokio::sync::mpsc::Receiver<Vec<u8>>) {
    let socket = UdpSocket::bind("0.0.0.0:0").await.expect("Failed to bind UDP sender socket");
    
    while let Some(data) = rx.recv().await {
        if let Err(e) = socket.send_to(&data, &target_addr).await {
            error!("Failed to send UDP packet to {}: {}", target_addr, e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::proto::{SensorReading, Header, SensorBatch};
    use shared::MessageWrapper;
    use std::time::Duration;

    #[tokio::test]
    async fn test_udp_listener_integration() {
        // 1. Setup AppState
        let (udp_tx, _udp_rx) = tokio::sync::mpsc::channel(10);
        let state = AppState::new(udp_tx);
        let rx_state = state.clone();

        // 2. Spawn UDP Listener on a test port
        let port = 5555;
        tokio::spawn(async move {
            if let Err(e) = udp_listener(rx_state, port).await {
                eprintln!("UDP listener error: {}", e);
            }
        });

        // Give it a moment to bind
        tokio::time::sleep(Duration::from_millis(100)).await;

        // 3. Create a sender socket
        let sender = UdpSocket::bind("0.0.0.0:0").await.expect("Failed to bind sender");
        sender.connect(format!("127.0.0.1:{}", port)).await.expect("Failed to connect");

        // 4. Create a SensorBatch message
        let msg = SensorBatch {
            header: Some(Header {
                source: "test_source".to_string(),
                dest: "".to_string(),
                seq: 1,
                timestamp: None,
                frame_id: "".to_string(),
                qos: None,
            }),
            readings: vec![
                SensorReading {
                    sensor_id: "temp_1".to_string(),
                    scalar: 25.5,
                    r#type: 0, // TYPE_UNSPECIFIED
                    ..Default::default()
                }
            ],
        };

        // Encode it
        let buf = MessageWrapper::SensorBatch(msg.clone()).to_bytes().expect("Failed to encode");

        // 5. Send it
        sender.send(&buf).await.expect("Failed to send");

        // 6. Verify state update
        // Subscribe to broadcast channel
        let mut rx = state.tx.subscribe();

        // Wait for the message (with timeout)
        let received = tokio::time::timeout(Duration::from_secs(1), rx.recv()).await;
        
        match received {
            Ok(Ok(MessageWrapper::SensorBatch(received_msg))) => {
                assert_eq!(received_msg.header.unwrap().source, "test_source");
                assert_eq!(received_msg.readings.len(), 1);
                assert_eq!(received_msg.readings[0].scalar, 25.5);
            }
            Ok(Ok(_)) => panic!("Received wrong message type"),
            Ok(Err(e)) => panic!("Broadcast receive error: {}", e),
            Err(_) => panic!("Timed out waiting for message"),
        }
    }
}
