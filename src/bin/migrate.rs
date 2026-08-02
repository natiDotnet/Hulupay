use toasty_cli::{Config, ToastyCli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");

    let db = Rust::db::build_toasty_db(&db_url).await?;
    let config = Config::load()?;
    ToastyCli::with_config(db, config).parse_and_run().await?;
    Ok(())
}
