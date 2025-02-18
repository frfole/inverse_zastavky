use crate::App;
use anyhow::anyhow;
use sqlx::sqlite::SqliteRow;
use sqlx::{query, Pool, Row, Sqlite};

impl App {
    pub async fn print_stats(db_pool: &Pool<Sqlite>) -> anyhow::Result<()> {
        let rows: Vec<SqliteRow> = query(
            "
SELECT count(*) FROM el_station_names;
SELECT count(*) FROM el_station_pos;
SELECT count(*) FROM hl_stations;
",
        )
        .fetch_all(db_pool)
        .await?;
        let station_names: u32 = rows.get(0).ok_or(anyhow!(""))?.try_get(0)?;
        let station_poses: u32 = rows.get(1).ok_or(anyhow!(""))?.try_get(0)?;
        let stations_done: u32 = rows.get(2).ok_or(anyhow!(""))?.try_get(0)?;
        println!("station names: {}", station_names);
        println!("station poses: {}", station_poses);
        println!("assigned chain stations: {}", stations_done);
        Ok(())
    }
}
