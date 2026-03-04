use crate::application::Claims;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

#[derive(Clone)]
pub struct AuthUser(pub Claims);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthUser>()
            .cloned()
            .map(|u| AuthUser(u.0))
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}
