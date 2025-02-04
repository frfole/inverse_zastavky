use crate::database::MainDB;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{get, FromForm};
use rocket_db_pools::sqlx::sqlite::SqliteRow;
use rocket_db_pools::sqlx::{Error, FromRow, Row};
use rocket_db_pools::{sqlx, Connection};

#[derive(FromForm)]
pub struct ListSlChains {
    limit: Option<u32>,
    page: Option<u32>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SlChain {
    chain_hash: String,
    station: String,
    pos: u32,
}

impl ListSlChains {
    pub fn limit(&self) -> u32 {
        self.limit.unwrap_or(u32::MAX).min(50)
    }

    pub fn page(&self) -> u32 {
        self.page.unwrap_or(0)
    }
}

impl FromRow<'_, SqliteRow> for SlChain {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, Error> {
        Ok(SlChain {
            chain_hash: row.try_get("chain_hash")?,
            station: row.try_get("station_name")?,
            pos: row.try_get("pos")?,
        })
    }
}

#[get("/sl_chains?<options..>")]
pub async fn list_sl_chains(
    mut db: Connection<MainDB>,
    options: ListSlChains,
) -> Result<Json<Vec<SlChain>>, String> {
    let limit = options.limit();
    let page = options.page();
    let chains = sqlx::query_as::<_, SlChain>("SELECT * FROM sl_chains LIMIT ? OFFSET ?")
        .bind(limit)
        .bind(page * limit)
        .fetch_all(&mut **db)
        .await;
    if let Err(e) = chains {
        return Err(format!("{}", e));
    }
    return Ok(Json(chains.unwrap()));
}
