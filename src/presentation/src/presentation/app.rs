use crate::presentation::routes::AppRoutes;
use crate::presentation::state::provide_app_state;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    provide_app_state();
    view! {
        <div class="dark bg-slate-950 text-slate-100 min-h-screen">
            <AppRoutes />
        </div>
    }
}
