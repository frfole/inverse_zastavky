use crate::model::{BBox, Station, StopId};
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
