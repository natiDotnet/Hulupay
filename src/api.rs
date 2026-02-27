use std::sync::Arc;
use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;
use application::merchant::repository::MerchantRepository;
use infrastructure::persistence::merchant_repository_impl::MerchantRepositoryPostgres;

pub mod merchant;
mod error;

#[derive(OpenApi)]
#[openapi(info(title = "My API", version = "1.0", description = "An example API"))]
pub struct ApiDoc;


pub fn api_routes(repo: Arc<dyn MerchantRepository>) -> Router {
    let (app, doc) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api", merchant::router(repo)
        )
        .split_for_parts();

    app.merge(
        SwaggerUi::new("/swagger-ui")
        .url("/api-doc/openapi.json", doc.clone()))
        .merge(Scalar::with_url("/scalar", doc.clone()))
}