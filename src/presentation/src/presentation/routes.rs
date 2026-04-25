use crate::presentation::layout::AppLayout;
use crate::presentation::pages::{
    DashboardPage, LoginPage, NotificationsPage, SettingsPage, UsersPage,
};
use crate::presentation::state::AuthState;
use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute, Route, Router, Routes};

#[component]
fn ProtectedRoute() -> impl IntoView {
    let auth = expect_context::<AuthState>();

    view! {
        <Show
            when=move || auth.is_authenticated.get()
            fallback=|| view! { <LoginPage /> }
        >
            <AppLayout>
                <Outlet />
            </AppLayout>
        </Show>
    }
}

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <p class="text-slate-300 p-6">"Not found"</p> }>
                <ParentRoute path=leptos_router::path!("") view=ProtectedRoute>
                    <Route path=leptos_router::path!("") view=DashboardPage />
                    <Route path=leptos_router::path!("users") view=UsersPage />
                    <Route path=leptos_router::path!("settings") view=SettingsPage />
                    <Route path=leptos_router::path!("notifications") view=NotificationsPage />
                </ParentRoute>
                <Route path=leptos_router::path!("/login") view=LoginPage />
            </Routes>
        </Router>
    }
}
