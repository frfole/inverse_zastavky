use serde::Serialize;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Sqlite, query};

#[derive(Serialize)]
pub struct Stats {
    pos_count: i32,
    names_count: i32,
}

impl Stats {
    pub fn new(pos_count: i32, names_count: i32) -> Stats {
        Stats {
            pos_count,
            names_count,
        }
    }

    pub async fn get(db: &mut PoolConnection<Sqlite>) -> Result<Self, sqlx::Error> {
        let rows: Vec<SqliteRow> = query(
            "SELECT count(*) FROM el_station_pos;
    SELECT count(*) FROM el_station_names;
    ",
        )
        .fetch_all(&mut **db)
        .await?;
        let pos_count = rows.get(0).unwrap().try_get(0)?;
        let names_count = rows.get(1).unwrap().try_get(0)?;
        Ok(Stats::new(pos_count, names_count))
    }
}
