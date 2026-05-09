pub mod api;
use crate::api::api_routes;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().json().with_target(false))
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    let app = api_routes(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;
    axum::serve(listener, app).await?;
    println!("Server started");
    Ok(())
}
