use std::env;
use axum::{Extension, Router};
use std::sync::Arc;
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;
use utoipa_axum::router::{OpenApiRouter}; // Import NestedApiConfig
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;
use application::auth::token::TokenService;
use infrastructure::auth::jwt_token_service::JwtTokenService;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::openapi::SecurityRequirement;
use crate::api::middleware::authentication;

pub mod merchant;
mod error;
pub mod auth;
mod middleware;

#[derive(utoipa::OpenApi)]
#[openapi(
    info(title = "My API", version = "1.0", description = "An example API"),
)]
pub struct ApiDoc;


pub fn api_routes(pool: Pool<Postgres>) -> Router {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_service: Arc<dyn TokenService> = Arc::new(JwtTokenService::new(jwt_secret));

    let mut open_api = ApiDoc::openapi();
    open_api.components
        .get_or_insert_default()
        .security_schemes
        .insert(
            "bearerAuth".to_string(),
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );

    let (app, doc) = OpenApiRouter::with_openapi(open_api)
        .nest("/api",
              OpenApiRouter::new()
                  .merge(merchant::router(pool.clone()))
                  .merge(auth::router(pool.clone()))
        )
        .layer(axum::middleware::from_fn(authentication))
        .layer(Extension(token_service.clone()))
        .split_for_parts();

    app.merge(
        SwaggerUi::new("/swagger-ui")
        .url("/api-doc/openapi.json", doc.clone()))
        .merge(Scalar::with_url("/scalar", doc.clone()))
}