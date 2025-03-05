use crate::model::bbox::BBox;
use serde::Serialize;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Sqlite, query};

#[derive(Serialize)]
pub struct BaseStation {
    lat: f64,
    lon: f64,
    name: String,
}

impl BaseStation {
    pub fn new(lat: f64, lon: f64, name: String) -> BaseStation {
        BaseStation { lat, lon, name }
    }

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
