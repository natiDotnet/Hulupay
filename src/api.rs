use axum::routing::get;
use axum::Router;
use std::env;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

// Feature crate routers
use auth;
use auth::{JwtTokenService, TokenService};
use auth::api::get_token_service;
use merchant;
use payments;

#[derive(utoipa::OpenApi)]
#[openapi(info(title = "My API", version = "1.0", description = "An example API"))]
pub struct ApiDoc;


pub fn api_routes(toasty_db: &toasty::Db) -> Router {

    let token_service = get_token_service();
    let mut open_api = ApiDoc::openapi();
    open_api
        .components
        .get_or_insert_default()
        .security_schemes
        .insert(
            "bearer_auth".to_string(),
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );

    seed_database(toasty_db);

    // Configure CORS to allow all origins (for development)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let (app, doc) = OpenApiRouter::with_openapi(open_api)
        .nest(
            "/api",
            OpenApiRouter::new()
                // Auth routes (no auth required for login/register)
                .merge(auth::router(toasty_db))
                // Merchant routes (requires MasterAdmin role)
                .merge(merchant::router(toasty_db))
                // Payment routes (requires authentication)
                .merge(OpenApiRouter::new().route("/test", get(|| async { "Hello, World!" }))),
        )
        .layer(axum::middleware::from_fn(auth::api::authentication))
        .layer(axum::Extension(token_service.clone()))
        .layer(axum::Extension(toasty_db.clone()))
        .merge(payments::router(toasty_db))
        .split_for_parts();

    app
        .merge(Scalar::with_url("/scalar", doc.clone()))
        .layer(cors)
}

pub fn seed_database(toasty_db: &toasty::Db) {
    let seeder = payments::infrastructure::seed::DataSeeder::new(toasty_db.clone());
    tokio::spawn(async move {
        if let Err(err) = seeder.seed().await {
            tracing::error!("Failed to run seeder: {:?}", err);
        }
    });
}
