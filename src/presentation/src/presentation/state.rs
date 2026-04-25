use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct UiState {
    pub sidebar_collapsed: RwSignal<bool>,
    pub dark_mode: RwSignal<bool>,
}

#[derive(Clone, Debug)]
pub struct AuthState {
    pub is_authenticated: RwSignal<bool>,
}

pub fn provide_app_state() {
    provide_context(UiState {
        sidebar_collapsed: RwSignal::new(false),
        dark_mode: RwSignal::new(true),
    });
    provide_context(AuthState {
        is_authenticated: RwSignal::new(true),
    });
}
