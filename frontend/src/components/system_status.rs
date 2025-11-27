use leptos::*;
use shared::proto::SystemStatus;

#[component]
pub fn SystemStatusPanel(
    #[prop(into)]
    status: Signal<Option<SystemStatus>>,
    #[prop(into)]
    connected: Signal<bool>,
) -> impl IntoView {
    view! {
        <div class="system-status card">
            <h2>"System Status"</h2>
            <div class="status-details">
                <div class="status-row">
                    <span class="label">"Connection"</span>
                    <span class={move || if connected.get() { "value connected" } else { "value disconnected" }}>
                        {move || if connected.get() { "Connected" } else { "Disconnected" }}
                    </span>
                </div>
                {move || {
                    match status.get() {
                        Some(s) => view! {
                            <div class="status-group">
                                <div class="status-row">
                                    <span class="label">"State"</span>
                                    <span class="value">{format!("{:?}", s.state)}</span>
                                </div>
                                {s.metrics.into_iter().map(|(k, v)| {
                                    view! {
                                        <div class="status-row">
                                            <span class="label">{k}</span>
                                            <span class="value">{format!("{:.2}", v)}</span>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_view(),
                        None => view! { <div class="waiting">"No system status received"</div> }.into_view()
                    }
                }}
            </div>
        </div>
    }
}
