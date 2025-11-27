use leptos::*;
use shared::proto::SensorBatch;

#[component]
pub fn SensorDisplay(
    #[prop(into)]
    data: Signal<Option<SensorBatch>>,
) -> impl IntoView {
    view! {
        <div class="sensor-display card">
            <h2>"Sensor Data"</h2>
            {move || {
                match data.get() {
                    Some(batch) => view! {
                        <div class="sensor-grid">
                            {batch.readings.into_iter().map(|reading| {
                                view! {
                                    <div class="sensor-item">
                                        <span class="sensor-id">{reading.sensor_id}</span>
                                        <span class="sensor-value">{format!("{:.2}", reading.scalar)}</span>

                                        <span class="sensor-unit">{reading.units}</span>

                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_view(),
                    None => view! { <div class="waiting">"Waiting for sensor data..."</div> }.into_view()
                }
            }}
        </div>
    }
}
