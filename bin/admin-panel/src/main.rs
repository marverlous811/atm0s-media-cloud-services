use std::sync::Arc;

use atm0s_cloud_admin_panel::{database, http};
use clap::Parser;
use tracing_subscriber::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(env, long, default_value_t = 9090)]
    port: u16,

    #[arg(env, long, default_value = "sqlite:admin-panel.db?mode=rwc")]
    database_url: String,

    #[arg(env, long, default_value = "insecure")]
    cluster_secret: String,

    #[arg(env, long)]
    clerk_secret: String,
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

    let client: Arc<dyn welds::Client> = if args.database_url.starts_with("sqlite") {
        log::info!("Using sqlite database");
        let client = welds::connections::sqlite::connect(&args.database_url).await?;
        database::migrations::migration_up(&client).await?;
        database::migrations::check_tables(&client).await?;
        Arc::from(client)
    } else if args.database_url.starts_with("postgres") {
        log::info!("Using postgres database");
        let client = welds::connections::postgres::connect(&args.database_url).await?;
        database::migrations::migration_up(&client).await?;
        database::migrations::check_tables(&client).await?;
        Arc::from(client)
    } else if args.database_url.starts_with("mysql") {
        log::info!("Using mysql database");
        let client = welds::connections::mysql::connect(&args.database_url).await?;
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
            cluster_secret: args.cluster_secret,
            clerk_secret: args.clerk_secret,
        },
    )
    .await?;

    Ok(())
}
