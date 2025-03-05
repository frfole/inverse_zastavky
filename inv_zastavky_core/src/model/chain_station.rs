use crate::model::StopId;
use serde::Serialize;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Sqlite, query};

#[derive(Serialize, Debug)]
pub struct ChainStation {
    chain_hash: String,
    name: String,
    pos: i32,
    stop_id: Option<StopId>,
}

impl ChainStation {
    pub fn new(chain_hash: String, name: String, pos: i32, stop_id: Option<StopId>) -> Self {
        Self {
            chain_hash,
            name,
            pos,
            stop_id,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let chain_hash = row.try_get(0)?;
        let name = row.try_get(1)?;
        let pos = row.try_get(2)?;
        let stop_id = if row.len() >= 4 {
            row.try_get(3)?
        } else {
            None
        };
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

    pub async fn get_by_chain_hash(
        db: &mut PoolConnection<Sqlite>,
        chain_hash: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let rows: Vec<SqliteRow> =
            query("SELECT sl_chains.chain_hash, sl_chains.station_name, sl_chains.pos, hl_stations.stop_id FROM sl_chains
LEFT JOIN hl_stations
    ON hl_stations.chain_hash = sl_chains.chain_hash AND hl_stations.pos = sl_chains.pos WHERE sl_chains.chain_hash = ?
ORDER BY sl_chains.chain_hash, sl_chains.pos")
                .bind(chain_hash.to_string())
                .fetch_all(&mut **db)
                .await?;
        let mut stations = Vec::new();
        for row in rows {
            stations.push(Self::from_row(&row)?);
        }
        Ok(stations)
    }
}
