use application::auth::claims::Claims;
use application::auth::token::TokenService;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthUser(pub Claims);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthUser>()
            .cloned()
            .map(|u| AuthUser(u.0))
            .ok_or(StatusCode::UNAUTHORIZED)

        // let token_service = parts
        //     .extensions
        //     .get::<Arc<dyn TokenService>>()
        //     .cloned()
        //     .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        //
        // let TypedHeader(Authorization(bearer)) =
        //     TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
        //         .await
        //         .map_err(|_| StatusCode::UNAUTHORIZED)?;
        //
        // let claims = token_service
        //     .validate(bearer.token())
        //     .map_err(|_| StatusCode::UNAUTHORIZED)?;
        //
        // Ok(AuthUser(claims))
    }
}