mod database;
mod export;
mod import;
mod netex;
mod stats;

use clap::{Parser, Subcommand};
use inv_zastavky_core::suggest::chain::{chain_options, path_options};
use std::path::PathBuf;

struct App;

#[derive(Parser)]
struct Cli {
    #[arg(value_name = "URL", help = "SQLite file path")]
    db_url: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Import {
        #[arg(
            long,
            value_name = "NETEX FILE",
            help = "Path pointing to ZIP of Netex files"
        )]
        netex_path: Option<PathBuf>,
        #[arg(
            long,
            value_name = "GEOJSON FILE",
            help = "Path pointing to GeoJSON encoded list of base stations"
        )]
        base_stations: Option<PathBuf>,
        #[arg(
            long,
            value_name = "GEOJSON FILE",
            help = "Path pointing to GeoJSON encoded list of cities"
        )]
        base_cities: Option<PathBuf>,
    },
    Export {
        #[arg(
            value_name = "OUTPUT GEOJSON FILE",
            help = "GeoJSON encoded list of stations"
        )]
        output_file: PathBuf,
    },
    Stats {},
    Dev {
        #[arg()]
        chain_hash: String,
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
        Commands::Stats {} => {
            App::print_stats(&db_pool).await?;
        }
        Commands::Dev { chain_hash } => {
            for suggestion in path_options(&mut db_pool.acquire().await?, &chain_hash).await? {
                println!("cities: {:?}", suggestion)
            }
            for suggestion in chain_options(&mut db_pool.acquire().await?, &chain_hash).await? {
                println!("stations: {:?}", suggestion)
            }
        }
    }
    Ok(())
}
