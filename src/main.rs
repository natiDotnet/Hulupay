pub mod api;

use crate::api::merchant;
use dotenvy::dotenv;
use infrastructure::persistence::merchant_repository_impl::MerchantRepositoryPostgres;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer()
            .json()
            .with_target(false))
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;
    let repo= Arc::new(MerchantRepositoryPostgres::new(pool));

    let (app, doc) = OpenApiRouter::new()
        .routes(merchant::router(repo.clone()))
        .split_for_parts();

    let app = app
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-doc/openapi.json", doc.clone()))
        .merge(Scalar::with_url("/scalar", doc.clone()));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}