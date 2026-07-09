use axum::routing::get;
use axum::Router;
use sea_orm::DatabaseConnection;
use std::env;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};
// use utoipa_swagger_ui::SwaggerUi;

// Feature crate routers
use auth;
use auth::{JwtTokenService, TokenService};
use auth::api::get_token_service;
use merchant;
use payments;

#[derive(utoipa::OpenApi)]
#[openapi(info(title = "My API", version = "1.0", description = "An example API"))]
pub struct ApiDoc;


pub fn api_routes(db: &DatabaseConnection) -> Router {
    
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

    seed_database(db);

    // Configure CORS to allow all origins (for development)
    // In production, you should restrict this to specific origins
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    // .allow_credentials(true);

    let (app, doc) = OpenApiRouter::with_openapi(open_api)
        .nest(
            "/api",
            OpenApiRouter::new()
                // Auth routes (no auth required for login/register)
                .merge(auth::router(db))
                // Merchant routes (requires MasterAdmin role)
                .merge(merchant::router(db))
                // Payment routes (requires authentication)
                // .merge(payments::router(db))
                // .merge(payments::chapa::router(db))
                .merge(OpenApiRouter::new().route("/test", get(|| async { "Hello, World!" }))),
        )
        .layer(axum::middleware::from_fn(auth::api::authentication))
        .layer(axum::Extension(token_service.clone()))
        .merge(payments::router(db))
        .split_for_parts();

    app
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", doc.clone()))
        .merge(Scalar::with_url("/scalar", doc.clone()))
        .layer(cors)
}

pub fn seed_database(db: &DatabaseConnection) {
    let seeder = payments::infrastructure::seed::DataSeeder::new(db.clone());
    tokio::spawn(async move {
        if let Err(err) = seeder.seed().await {
            tracing::error!("Failed to run seeder: {:?}", err);
        }
    });
}
