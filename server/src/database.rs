use crate::model::{BBox, BaseCity, BaseStation, ChainStation, Station, StopId};
use rocket_db_pools::sqlx::pool::PoolConnection;
use rocket_db_pools::sqlx::sqlite::SqliteRow;
use rocket_db_pools::sqlx::{query, Row, Sqlite};
use rocket_db_pools::{sqlx, Database};
use std::collections::HashMap;

#[derive(Database)]
#[database("main")]
pub struct MainDB(sqlx::SqlitePool);

impl Station {
    fn from_rows(stop_id: StopId, rows: &Vec<SqliteRow>) -> Result<Option<Self>, sqlx::Error> {
        let mut loc: Option<(f64, f64)> = None;
        let mut names: Vec<String> = Vec::new();
        for row in rows {
            let row_id: StopId = row.try_get(0)?;
            if row_id == stop_id {
                loc = Some((row.try_get(1)?, row.try_get(2)?));
                names.push(row.try_get(3)?);
            }
        }
        if let Some((lat, lon)) = loc {
            Ok(Some(Self::new(stop_id, names, lat, lon)))
        } else {
            Ok(None)
        }
    }

    pub async fn get_by_bbox(
        db: &mut PoolConnection<Sqlite>,
        bbox: &BBox,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let rows: Vec<SqliteRow> = query(
            "SELECT stop_id, lat, lon, station_name FROM el_station_names
    JOIN el_station_pos USING (stop_id) WHERE $1 <= lat AND lat <= $2 AND $3 <= lon AND lon <= $4 LIMIT 500;")
            .bind(bbox.lat_from())
            .bind(bbox.lat_to())
            .bind(bbox.lon_from())
            .bind(bbox.lon_to())
            .fetch_all(&mut **db).await?;
        let mut stations_names: HashMap<StopId, Vec<String>> = HashMap::new();
        let mut stations_locs: HashMap<StopId, (f64, f64)> = HashMap::new();
        for row in rows {
            let stop_id: StopId = row.try_get(0)?;
            if !stations_names.contains_key(&stop_id) {
                stations_locs.insert(stop_id, (row.try_get(1)?, row.try_get(2)?));
                stations_names.insert(stop_id, Vec::new());
            }
            stations_names
                .get_mut(&stop_id)
                .unwrap()
                .push(row.try_get(3)?);
        }
        let mut stations = Vec::new();
        for (stop_id, names) in stations_names {
            stations.push(Station::new(
                stop_id,
                names,
                stations_locs[&stop_id].0,
                stations_locs[&stop_id].1,
            ))
        }
        Ok(stations)
    }

    pub async fn get_by_id(
        db: &mut PoolConnection<Sqlite>,
        stop_id: StopId,
    ) -> Result<Option<Self>, sqlx::Error> {
        let rows: Vec<SqliteRow> = query(
            "SELECT stop_id, lat, lon, station_name FROM el_station_names
    JOIN el_station_pos USING (stop_id) WHERE stop_id = $1 LIMIT 1;",
        )
        .bind(stop_id)
        .fetch_all(&mut **db)
        .await?;
        Self::from_rows(stop_id, &rows)
    }

    pub async fn create_station(
        db: &mut PoolConnection<Sqlite>,
        stop_id: StopId,
        lat: f64,
        lon: f64,
        station_name: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let rows: Vec<SqliteRow> = query(
            "INSERT INTO el_station_pos (stop_id, lat, lon) VALUES ($1, $2, $3);
INSERT INTO el_station_names (stop_id, station_name) VALUES ($1, $4);
SELECT stop_id, lat, lon, station_name FROM el_station_names
    JOIN el_station_pos USING (stop_id) WHERE stop_id == $1;",
        )
        .bind(stop_id)
        .bind(lat)
        .bind(lon)
        .bind(station_name)
        .fetch_all(&mut **db)
        .await?;
        Self::from_rows(stop_id, &rows)
    }

    pub async fn remove_station(
        db: &mut PoolConnection<Sqlite>,
        stop_id: StopId,
    ) -> Result<(), sqlx::Error> {
        query(
            "DELETE FROM el_station_pos WHERE stop_id == $1;
DELETE FROM el_station_names WHERE stop_id == $1;",
        )
        .bind(stop_id)
        .execute(&mut **db)
        .await?;
        Ok(())
    }

    pub async fn move_station(
        db: &mut PoolConnection<Sqlite>,
        stop_id: StopId,
        lat: f64,
        lon: f64,
    ) -> Result<Option<Self>, sqlx::Error> {
        let rows: Vec<SqliteRow> = query(
            "UPDATE el_station_pos SET lat = $2, lon = $3 WHERE stop_id == $1;
SELECT stop_id, lat, lon, station_name FROM el_station_names
    JOIN el_station_pos USING (stop_id) WHERE stop_id == $1;",
        )
        .bind(stop_id)
        .bind(lat)
        .bind(lon)
        .fetch_all(&mut **db)
        .await?;
        Self::from_rows(stop_id, &rows)
    }

    pub async fn add_name(
        db: &mut PoolConnection<Sqlite>,
        stop_id: StopId,
        station_name: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let rows: Vec<SqliteRow> = query(
            "INSERT INTO el_station_names (stop_id, station_name) VALUES ($1, $2);
SELECT stop_id, lat, lon, station_name FROM el_station_names
    JOIN el_station_pos USING (stop_id) WHERE stop_id == $1;",
        )
        .bind(stop_id)
        .bind(station_name)
        .fetch_all(&mut **db)
        .await?;
        Self::from_rows(stop_id, &rows)
    }

    pub async fn remove_station_name(
        db: &mut PoolConnection<Sqlite>,
        stop_id: StopId,
        station_name: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let rows: Vec<SqliteRow> = query(
            "DELETE FROM el_station_names WHERE stop_id == $1 AND station_name == $2;
SELECT stop_id, lat, lon, station_name FROM el_station_names
    JOIN el_station_pos USING (stop_id) WHERE stop_id == $1;",
        )
        .bind(stop_id)
        .bind(station_name)
        .fetch_all(&mut **db)
        .await?;
        Self::from_rows(stop_id, &rows)
    }
}

impl ChainStation {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let chain_hash = row.try_get(0)?;
        let name = row.try_get(1)?;
        let pos = row.try_get(2)?;
        let stop_id = row.try_get(3)?;
        Ok(ChainStation::new(chain_hash, name, pos, stop_id))
    }

    pub async fn get_chains(
        db: &mut PoolConnection<Sqlite>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let rows = query("SELECT sl_chains.chain_hash, sl_chains.station_name, sl_chains.pos, hl_stations.stop_id FROM sl_chains
    LEFT JOIN hl_stations
        ON hl_stations.chain_hash = sl_chains.chain_hash AND hl_stations.pos = sl_chains.pos
ORDER BY sl_chains.chain_hash, sl_chains.pos LIMIT $1 OFFSET $2;")
            .bind(limit)
            .bind(offset)
            .fetch_all(&mut **db)
            .await?;
        let mut stations = Vec::new();
        for row in rows {
            stations.push(Self::from_row(&row)?);
        }
        Ok(stations)
    }
}

impl BaseStation {
    pub async fn get_by_bbox(
        db: &mut PoolConnection<Sqlite>,
        bbox: &BBox,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let rows: Vec<SqliteRow> = query(
            "SELECT lat, lon, station_name FROM sl_base_stations WHERE $1 <= lat AND lat <= $2 AND $3 <= lon AND lon <= $4 LIMIT 500;")
            .bind(bbox.lat_from())
            .bind(bbox.lat_to())
            .bind(bbox.lon_from())
            .bind(bbox.lon_to())
            .fetch_all(&mut **db).await?;
        let mut stations = Vec::new();
        for row in rows {
            let lat = row.try_get(0)?;
            let lon = row.try_get(1)?;
            let name = row.try_get(2)?;
            stations.push(Self::new(lat, lon, name));
        }
        Ok(stations)
    }
}

impl BaseCity {
    pub async fn search(
        db: &mut PoolConnection<Sqlite>,
        search: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let rows: Vec<SqliteRow> = query(
            "SELECT city_name, lat, lon FROM sl_base_cities WHERE city_name LIKE $1 ORDER BY city_name LIMIT 50;")
            .bind(search)
            .fetch_all(&mut **db).await?;
        let mut cities = Vec::new();
        for row in rows {
            let name = row.try_get(0)?;
            let lat = row.try_get(1)?;
            let lon = row.try_get(2)?;
            cities.push(Self::new(name, lat, lon));
        }
        Ok(cities)
    }
}

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
            .bind(station.stop_id)
            .execute(&mut **db)
            .await?;
    }
    Ok(station)
}
