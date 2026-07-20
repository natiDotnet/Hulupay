// use auth::api::AuthUser;
// use axum::body::Body;
// use axum::extract::{Request, State};
// use axum::middleware;
// use axum::{http::StatusCode, middleware::Next, response::Response};
// use utoipa_axum::router::OpenApiRouter;
//
// pub async fn authentication(
//     mut req: Request<Body>,
//     next: Next,
// ) -> Result<Response, StatusCode> {
//     // For now, we'll skip token validation as it's handled by the auth crate
//     // This can be extended later if needed
//     Ok(next.run(req).await)
// }
//
// pub async fn authorization(
//     State(policy): State<AuthorizationPolicy>,
//     req: Request<Body>,
//     next: Next,
// ) -> Result<Response, StatusCode> {
//     let user = req
//         .extensions()
//         .get::<AuthUser>()
//         .ok_or(StatusCode::UNAUTHORIZED)?;
//
//     match policy {
//         AuthorizationPolicy::Authenticated => {}
//         AuthorizationPolicy::Role(required_role) => {
//             if user.0.role != required_role.as_str() {
//                 return Err(StatusCode::FORBIDDEN);
//             }
//         }
//     }
//
//     Ok(next.run(req).await)
// }
//
// #[derive(Clone)]
// pub enum AuthorizationPolicy {
//     Authenticated,
//     Role(auth::Role),
// }
//
// pub trait AuthRouterExt {
//     fn require_auth(self) -> Self;
//     fn require_role(self, role: auth::Role) -> Self;
// }
//
// impl<S> AuthRouterExt for OpenApiRouter<S>
// where
//     S: Clone + Send + Sync + 'static,
// {
//     fn require_auth(self) -> Self {
//         self
//             .layer(middleware::from_fn_with_state(
//                 AuthorizationPolicy::Authenticated,
//                 authorization))
//     }
//
//     fn require_role(self, role: auth::Role) -> Self {
//         self
//             .layer(middleware::from_fn_with_state(
//                 AuthorizationPolicy::Role(role),
//                 authorization))
//     }
// }
