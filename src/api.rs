use std::env;
use application::merchant::repository::MerchantRepository;
use axum::{Extension, Router};
use std::sync::Arc;
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;
use application::auth::token::TokenService;
use infrastructure::auth::jwt_token_service::JwtTokenService;

pub mod merchant;
mod error;
pub mod auth;
mod middleware;

#[derive(OpenApi)]
#[openapi(info(title = "My API", version = "1.0", description = "An example API"))]
pub struct ApiDoc;


pub fn api_routes(pool: Pool<Postgres>) -> Router {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_service: Arc<dyn TokenService> = Arc::new(JwtTokenService::new(jwt_secret));

    let (app, doc) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api",
              merchant::router(pool.clone())
                  .nest("/auth", auth::router(pool.clone()))
        )
        .layer(Extension(token_service.clone()))
        .split_for_parts();

    app.merge(
        SwaggerUi::new("/swagger-ui")
        .url("/api-doc/openapi.json", doc.clone()))
        .merge(Scalar::with_url("/scalar", doc.clone()))
}