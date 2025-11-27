use leptos::*;
use leptos_router::*;
use crate::components::dashboard::Dashboard;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="" view=Dashboard/>
                </Routes>
            </main>
        </Router>
    }
}
