use leptos::*;
use crate::services::websocket::WebSocketService;
use crate::components::{
    sensor_display::SensorDisplay,
    system_status::SystemStatusPanel,
    control_panel::ControlPanel,
};
use shared::{MessageWrapper, proto::{SensorBatch, SystemStatus}};

#[component]
pub fn Dashboard() -> impl IntoView {
    // Signals for state
    let (connected, set_connected) = create_signal(false);
    let (sensor_data, set_sensor_data) = create_signal::<Option<SensorBatch>>(None);
    let (system_status, set_system_status) = create_signal::<Option<SystemStatus>>(None);

    // WebSocket Service
    let ws_service = WebSocketService::new(move |msg| {
        match msg {
            MessageWrapper::SensorBatch(batch) => set_sensor_data.set(Some(batch)),
            MessageWrapper::SystemStatus(status) => set_system_status.set(Some(status)),
            MessageWrapper::Heartbeat(_) => set_connected.set(true), // Assume heartbeat means connected
            _ => leptos::logging::log!("Received other message: {:?}", msg),
        }
    });

    // Handle sending commands
    let send_command = Callback::new(move |cmd: MessageWrapper| {
        ws_service.send(cmd);
    });

    // Effect to check connection (simple timeout logic could be added here)
    create_effect(move |_| {
        // Initial connection check or periodic ping could go here
    });

    view! {
        <div class="dashboard-container">
            <header>
                <h1>"Simulation Dashboard"</h1>
                // Placeholder for potential header actions or status summary
                <div class="header-actions">
                    // Could add theme toggle or connection status summary here
                </div>
            </header>
            
            <main class="dashboard-grid">
                <div class="left-panel">
                    <SystemStatusPanel status=system_status connected=connected />
                    <ControlPanel on_command=send_command />
                </div>
                
                <div class="right-panel">
                    <SensorDisplay data=sensor_data />
                </div>
            </main>
        </div>
    }
}
