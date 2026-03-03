use crate::api::auth::extractor::AuthUser;
use application::auth::token::TokenService;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::{http::StatusCode, middleware::Next, response::Response};
use domain::user::Role;
use std::sync::Arc;

pub async fn authentication(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Resolve token service from request extensions
    let token_service = req
        .extensions()
        .get::<Arc<dyn TokenService>>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Extract Authorization header manually
    let auth_header = match req.headers().get(AUTHORIZATION) {
        Some(value) => value,
        None => return Ok(next.run(req).await), // anonymous allowed
    };

    let auth_str = auth_header
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Expect "Bearer <token>"
    if !auth_str.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_str[7..];

    match token_service.validate(token) {
        Ok(claims) => {
            req.extensions_mut().insert(AuthUser(claims));
        }
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    }

    Ok(next.run(req).await)
}

use axum::middleware;
use utoipa_axum::router::OpenApiRouter;

pub async fn authorization(
    State(policy): State<AuthorizationPolicy>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {

    let user = req
        .extensions()
        .get::<AuthUser>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    match policy {
        AuthorizationPolicy::Authenticated => {}
        AuthorizationPolicy::Role(required_role) => {
            if user.0.role != required_role.as_str() {
                return Err(StatusCode::FORBIDDEN);
            }
        }
    }

    Ok(next.run(req).await)
}

#[derive(Clone)]
pub enum AuthorizationPolicy {
    Authenticated,
    Role(Role),
}

pub trait AuthRouterExt {
    fn require_auth(self) -> Self;
    fn require_role(self, role: Role) -> Self;
}

impl<S> AuthRouterExt for OpenApiRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn require_auth(self) -> Self {
        self
            .layer(middleware::from_fn_with_state(
                AuthorizationPolicy::Authenticated,
                authorization))
    }

    fn require_role(self, role: Role) -> Self {
        self
            .layer(middleware::from_fn_with_state(
                AuthorizationPolicy::Role(role),
                authorization))
    }
}