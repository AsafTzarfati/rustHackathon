use shared::models::MessageWrapper;
use shared::proto;
use std::net::SocketAddr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let bind_addr = "0.0.0.0:5001";
    let target_addr = std::env::var("BACKEND_HOST").unwrap_or_else(|_| "127.0.0.1:5000".to_string());

    let socket = UdpSocket::bind(bind_addr).await?;
    info!("Mock Realtime SW listening on {}", bind_addr);
    info!("Sending data to {}", target_addr);

    let target: SocketAddr = target_addr.parse()?;
    
    let mut interval = tokio::time::interval(Duration::from_millis(100)); // 10Hz
    let mut seq = 0;
    let start_time = SystemTime::now();

    loop {
        interval.tick().await;
        seq += 1;

        let now = SystemTime::now();
        let elapsed = now.duration_since(start_time).unwrap().as_secs_f64();
        
        // Create timestamp
        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let timestamp = Some(prost_types::Timestamp {
            seconds: since_epoch.as_secs() as i64,
            nanos: since_epoch.subsec_nanos() as i32,
        });

        // Generate physically sensible data
        // 1. Sine wave for scalar
        let sine_val = (elapsed * 0.5).sin() * 10.0 + 20.0; // Oscillates between 10 and 30
        
        // 2. Rotating vector (circular motion)
        let radius = 5.0;
        let angle = elapsed * 1.0; // 1 rad/s
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        let z = (elapsed * 0.1).sin() * 2.0;

        // 3. Temperature (slowly rising)
        let temp_ambient = 25.0 + (elapsed * 0.05).sin() * 5.0;
        let temp_cpu = 45.0 + (elapsed * 0.1).sin() * 10.0;

        let readings = vec![
            proto::SensorReading {
                sensor_id: "sine_wave".to_string(),
                r#type: proto::sensor_reading::Type::Scalar as i32,
                scalar: sine_val,
                units: "V".to_string(),
                ..Default::default()
            },
            proto::SensorReading {
                sensor_id: "position_vector".to_string(),
                r#type: proto::sensor_reading::Type::Vector as i32,
                vector: vec![x, y, z],
                units: "m".to_string(),
                ..Default::default()
            },
            proto::SensorReading {
                sensor_id: "main_temp".to_string(),
                r#type: proto::sensor_reading::Type::Temperature as i32,
                temperature: Some(proto::TemperatureData {
                    ambient: temp_ambient,
                    cpu: temp_cpu,
                    board: temp_ambient + 2.0,
                }),
                units: "C".to_string(),
                ..Default::default()
            },
        ];

        let batch = proto::SensorBatch {
            header: Some(proto::Header {
                source: "mock_realtime".to_string(),
                dest: "backend".to_string(),
                seq,
                timestamp: timestamp.clone(),
                frame_id: "world".to_string(),
                qos: None,
            }),
            readings,
        };

        let wrapper = MessageWrapper::SensorBatch(batch);
        match wrapper.to_bytes() {
            Ok(bytes) => {
                if let Err(e) = socket.send_to(&bytes, target).await {
                    error!("Failed to send packet: {}", e);
                }
            }
            Err(e) => {
                error!("Failed to encode packet: {}", e);
            }
        }

        // Also send SystemStatus occasionally (every 10th frame, i.e., 1Hz)
        if seq % 10 == 0 {
            let status = proto::SystemStatus {
                header: Some(proto::Header {
                    source: "mock_realtime".to_string(),
                    dest: "backend".to_string(),
                    seq,
                    timestamp,
                    frame_id: "system".to_string(),
                    qos: None,
                }),
                state: proto::system_status::State::Running as i32,
                detail: "System is running normally".to_string(),
                metrics: std::collections::HashMap::from([
                    ("cpu_load".to_string(), 15.0 + (elapsed * 0.2).sin() * 5.0),
                    ("memory_usage".to_string(), 256.0),
                ]),
                rt: None,
            };
            
            let wrapper = MessageWrapper::SystemStatus(status);
             match wrapper.to_bytes() {
                Ok(bytes) => {
                    if let Err(e) = socket.send_to(&bytes, target).await {
                        error!("Failed to send status packet: {}", e);
                    }
                }
                Err(e) => {
                    error!("Failed to encode status packet: {}", e);
                }
            }
        }
    }
}
