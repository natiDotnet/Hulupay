pub mod api;

use crate::api::api_routes;
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().json().pretty().with_target(false))
        .init();

    let db = &Database::connect(db_url).await?;
    auto_apply(db).await?;

    let app = api_routes(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;
    axum::serve(listener, app).await?;
    println!("Server started");
    Ok(())
}

pub async fn auto_apply(db: &DatabaseConnection) -> anyhow::Result<()> {
    db.get_schema_registry("merchant::domain::*")
        .sync(db)
        .await?;
    db.get_schema_registry("auth::domain::*").sync(db).await?;
    // synchronizes database schema with entity definitions
    db.get_schema_registry("payments::domain::*")
        .sync(db)
        .await?;

    Ok(())
}
