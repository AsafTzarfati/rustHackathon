use leptos::*;
use shared::proto::{ActuatorCommand, actuator_command::Command};
use shared::MessageWrapper;
use std::collections::HashMap;

#[component]
pub fn ControlPanel(
    #[prop(into)]
    on_command: Callback<MessageWrapper>,
) -> impl IntoView {
    let send_test_command = move |_| {
        let cmd = ActuatorCommand {
            header: None,
            actuator_id: "test_actuator".to_string(),
            command: Some(Command::Value(1.0)),
            params: HashMap::new(),
        };
        on_command.call(MessageWrapper::ActuatorCommand(cmd));
    };

    view! {
        <div class="control-panel card">
            <h2>"Controls"</h2>
            <div class="button-group">
                <button class="btn primary" on:click=send_test_command>"Send Test Command"</button>
                // Add more controls here as needed
            </div>
        </div>
    }
}
