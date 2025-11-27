use leptos::*;
use gloo_net::websocket::{futures::WebSocket, Message};
use futures::{StreamExt, SinkExt};
use shared::proto::{ActuatorCommand, actuator_command::Command};

#[component]
pub fn Dashboard() -> impl IntoView {
    // State for sensor data
    let (sensor_data, set_sensor_data) = create_signal(String::from("Waiting for data..."));
    let (ws_sender, set_ws_sender) = create_signal::<Option<futures::channel::mpsc::Sender<String>>>(None);
    
    // WebSocket connection
    create_effect(move |_| {
        let location = web_sys::window().unwrap().location();
        let protocol = if location.protocol().unwrap() == "https:" { "wss" } else { "ws" };
        let host = location.host().unwrap();
        let ws_url = format!("{}://{}/ws", protocol, host);
        
        spawn_local(async move {
            if let Ok(ws) = WebSocket::open(&ws_url) {
                let (mut write, mut read) = ws.split();
                let (tx, mut rx) = futures::channel::mpsc::channel::<String>(10);
                set_ws_sender.set(Some(tx));

                spawn_local(async move {
                    while let Some(msg) = rx.next().await {
                        let _ = write.send(Message::Text(msg)).await;
                    }
                });

                while let Some(msg) = read.next().await {
                    if let Ok(Message::Text(text)) = msg {
                        set_sensor_data.set(text);
                    }
                }
            }
        });
    });

    let send_command = move |_| {
        if let Some(mut tx) = ws_sender.get() {
            spawn_local(async move {
                let cmd = ActuatorCommand {
                    header: None,
                    actuator_id: "test_actuator".to_string(),
                    command: Some(Command::Value(1.0)),
                    params: std::collections::HashMap::new(),
                };
                if let Ok(json) = serde_json::to_string(&cmd) {
                    let _ = tx.try_send(json);
                }
            });
        }
    };

    view! {
        <div class="dashboard">
            <h1>"Simulation Dashboard"</h1>
            <div class="controls">
                <button on:click=send_command>"Send Test Command"</button>
            </div>
            <div class="sensor-data">
                <h2>"Latest Data"</h2>
                <pre>{move || sensor_data.get()}</pre>
            </div>
        </div>
    }
}
