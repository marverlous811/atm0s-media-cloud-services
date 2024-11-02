use std::sync::Arc;

use admin_panel::{database, http};
use clap::Parser;
use tracing_subscriber::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(env, long, default_value_t = 8080)]
    port: u16,

    #[arg(env, long, default_value = "postgres://postgres:postgres@localhost:5432/postgres")]
    database_url: String,

    #[arg(env, long, default_value = "secret")]
    jwt_secret: String,

    #[arg(env, long, default_value = "60")]
    jwt_max_age_minutes: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let client: Arc<dyn welds::Client> = if args.database_url.starts_with("sqlite:") {
        log::info!("Using sqlite database");
        let client = welds::connections::sqlite::connect(&args.database_url).await?;
        database::migrations::migration_up(&client).await?;
        database::migrations::check_tables(&client).await?;
        Arc::from(client)
    } else if args.database_url.starts_with("postgres:") {
        log::info!("Using postgres database");
        let client = welds::connections::postgres::connect(&args.database_url).await?;
        database::migrations::migration_up(&client).await?;
        database::migrations::check_tables(&client).await?;
        Arc::from(client)
    } else {
        anyhow::bail!("Unsupported database url: {}", args.database_url)
    };

    http::run_http(
        args.port,
        client,
        http::HttpCfg {
            jwt_secret: args.jwt_secret,
            jwt_max_age_minutes: args.jwt_max_age_minutes,
        },
    )
    .await?;

    Ok(())
}
