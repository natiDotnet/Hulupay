use axum::Router;
use sqlx::{Pool, Postgres};
use std::env;
use std::sync::Arc;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;

// Feature crate routers
use auth;
use auth::{JwtTokenService, TokenService};
use merchant;
use payments;
use payments::application::arifpay::ArifPayConfig;

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
            "bearer_auth".to_string(),
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );

    // Create ArifPay provider for payments
    let arifpay_config = ArifPayConfig::new(
        env::var("ARIFPAY_API_KEY").unwrap_or_else(|_| "test_key".to_string()),
        env::var("ARIFPAY_IS_TEST_KEY").unwrap_or_else(|_| "true".to_string()) == "true",
    );
    let arifpay_provider = payments::infrastructure::ArifPayProvider::new(arifpay_config);

    let (app, doc) = OpenApiRouter::with_openapi(open_api)
        .nest("/api",
              OpenApiRouter::new()
                  // Auth routes (no auth required for login/register)
                  .merge(auth::router(pool.clone()))
                  // Merchant routes (requires MasterAdmin role)
                  .merge(merchant::router(pool.clone()))
                  // Payment routes (requires authentication)
                  .merge(payments::router(arifpay_provider, pool.clone()))
        )
        .layer(axum::middleware::from_fn(auth::api::authentication))
        .layer(axum::Extension(token_service.clone()))
        .split_for_parts();

    app.merge(
        SwaggerUi::new("/swagger-ui")
        .url("/api-doc/openapi.json", doc.clone()))
        .merge(Scalar::with_url("/scalar", doc.clone()))
}