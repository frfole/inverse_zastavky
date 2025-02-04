mod database;
mod import;
mod netex;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

struct App;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "URL")]
    db_url: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Import {
        #[arg(short, long, value_name = "FILE")]
        netex_path: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let db_pool = sqlx::sqlite::SqlitePool::connect(&cli.db_url).await?;
    database::ensure_tables(&db_pool).await?;
    match cli.command {
        Commands::Import { netex_path } => {
            App::import(&db_pool, netex_path).await?;
        }
    }
    Ok(())
}
