pub mod api;

use crate::api::api_routes;
use dotenvy::dotenv;
use std::env;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in the .env file");

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().json().pretty().with_target(false))
        .init();

    // Toasty pool — used by all crates (auth, merchant, payments)
    let toasty_db = Rust::db::build_toasty_db(&db_url).await?;

    let app = api_routes(&toasty_db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;
    axum::serve(listener, app).await?;
    println!("Server started");
    Ok(())
}
