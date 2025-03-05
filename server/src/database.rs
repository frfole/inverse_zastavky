use inv_zastavky_core::model::StopId;
use inv_zastavky_core::model::station::Station;
use rocket_db_pools::sqlx::pool::PoolConnection;
use rocket_db_pools::sqlx::{Sqlite, query};
use rocket_db_pools::{Database, sqlx};

#[derive(Database)]
#[database("main")]
pub struct MainDB(sqlx::SqlitePool);

pub async fn locate_chain_by_id(
    db: &mut PoolConnection<Sqlite>,
    chain: &str,
    name: &str,
    pos: i32,
    stop_id: StopId,
) -> Result<Option<Station>, sqlx::Error> {
    query(
        "INSERT OR REPLACE INTO hl_stations (chain_hash, station_name, pos, stop_id) VALUES ($1, $2, $3, $4);
INSERT OR IGNORE INTO el_station_names (stop_id, station_name) VALUES ($4, $2);",
    )
    .bind(chain)
    .bind(name)
    .bind(pos)
    .bind(stop_id)
    .execute(&mut **db)
    .await?;
    Station::get_by_id(db, stop_id).await
}

pub async fn locate_chain_by_loc(
    db: &mut PoolConnection<Sqlite>,
    chain: &str,
    name: &str,
    pos: i32,
    lat: f64,
    lon: f64,
) -> Result<Option<Station>, sqlx::Error> {
    let station =
        Station::create_station(db, rand::random::<i32>() as StopId, lat, lon, name).await?;
    if let Some(station) = &station {
        query(
            "INSERT OR REPLACE INTO hl_stations (chain_hash, station_name, pos, stop_id) VALUES ($1, $2, $3, $4);",
        )
            .bind(chain)
            .bind(name)
            .bind(pos)
            .bind(station.stop_id())
            .execute(&mut **db)
            .await?;
    }
    Ok(station)
}
