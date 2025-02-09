mod database;
mod export;
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
        #[arg(long, value_name = "NETEX FILE")]
        netex_path: Option<PathBuf>,
        #[arg(long, value_name = "GEOJSON FILE")]
        base_stations: Option<PathBuf>,
        #[arg(long, value_name = "GEOJSON FILE")]
        base_cities: Option<PathBuf>,
    },
    Export {
        #[arg(value_name = "OUTPUT GEOJSON FILE")]
        output_file: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let db_pool = sqlx::sqlite::SqlitePool::connect(&cli.db_url).await?;
    database::ensure_tables(&db_pool).await?;
    match cli.command {
        Commands::Import {
            netex_path,
            base_stations,
            base_cities,
        } => {
            if netex_path.is_some() {
                App::import(&db_pool, netex_path).await?;
            }
            if base_stations.is_some() {
                App::import_base_stations(&db_pool, base_stations.unwrap()).await?;
            }
            if base_cities.is_some() {
                App::import_base_cities(&db_pool, base_cities.unwrap()).await?;
            }
        }
        Commands::Export { output_file } => {
            App::export(&db_pool, output_file).await?;
        }
    }
    Ok(())
}
