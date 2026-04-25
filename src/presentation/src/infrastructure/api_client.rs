use crate::application::services;
use crate::domain::models::{ProfileSettings, SystemConfig, User};
use leptos::prelude::*;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ApiError {
    #[error("Unable to fetch data")]
    FetchFailure,
    #[error("Unable to save data")]
    SaveFailure,
}

#[server]
pub async fn fetch_users() -> Result<Vec<User>, ServerFnError> {
    services::seed_users().map_err(|_| ServerFnError::ServerError("seed error".into()))
}

#[server]
pub async fn save_user(user: User) -> Result<User, ServerFnError> {
    if user.email.is_empty() {
        return Err(ServerFnError::ServerError("email is required".into()));
    }
    Ok(user)
}

#[server]
pub async fn delete_user(_id: String) -> Result<(), ServerFnError> {
    Ok(())
}

#[server]
pub async fn load_profile_settings() -> Result<ProfileSettings, ServerFnError> {
    Ok(services::default_profile())
}

#[server]
pub async fn load_system_config() -> Result<SystemConfig, ServerFnError> {
    Ok(services::default_config())
}
