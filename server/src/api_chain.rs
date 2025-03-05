use crate::database;
use crate::database::MainDB;
use inv_zastavky_core::model::StopId;
use inv_zastavky_core::model::chain_station::ChainStation;
use inv_zastavky_core::model::station::Station;
use rocket::serde::json::Json;
use rocket::{FromForm, get};
use rocket_db_pools::Connection;

#[derive(FromForm)]
pub struct ListSlChainsParams {
    limit: Option<u32>,
    offset: Option<u32>,
}

#[derive(FromForm)]
pub struct LocateByIdParams {
    chain_hash: String,
    name: String,
    pos: i32,
    stop_id: StopId,
}

#[derive(FromForm)]
pub struct LocateByLocParams {
    chain_hash: String,
    name: String,
    pos: i32,
    lat: f64,
    lon: f64,
}

impl ListSlChainsParams {
    pub fn limit(&self) -> u32 {
        self.limit.unwrap_or(u32::MAX).min(50)
    }

    pub fn page(&self) -> u32 {
        self.offset.unwrap_or(0)
    }
}

#[get("/sl_chains?<params..>")]
pub async fn list_sl_chains(
    mut db: Connection<MainDB>,
    params: ListSlChainsParams,
) -> Result<Json<Vec<ChainStation>>, String> {
    let limit = params.limit();
    let offset = params.page();
    let chains = ChainStation::get_chains(&mut db, limit, offset).await;
    match chains {
        Ok(chains) => Ok(Json(chains)),
        Err(err) => Err(format!("{}", err)),
    }
}

#[get("/sl_chain?<chain_hash>")]
pub async fn sl_chain_by_chain_hash(
    mut db: Connection<MainDB>,
    chain_hash: String,
) -> Result<Json<Vec<ChainStation>>, String> {
    let chains = ChainStation::get_by_chain_hash(&mut db, &chain_hash).await;
    match chains {
        Ok(chains) => Ok(Json(chains)),
        Err(err) => Err(format!("{}", err)),
    }
}

#[get("/locate_by_id?<params..>")]
pub async fn locate_by_id(
    mut db: Connection<MainDB>,
    params: LocateByIdParams,
) -> Result<Json<Station>, String> {
    match database::locate_chain_by_id(
        &mut db,
        &params.chain_hash,
        &params.name,
        params.pos,
        params.stop_id,
    )
    .await
    {
        Ok(Some(station)) => Ok(Json(station)),
        Ok(None) => Err(String::from("no station found")),
        Err(err) => Err(format!("{}", err)),
    }
}

#[get("/locate_by_loc?<params..>")]
pub async fn locate_by_loc(
    mut db: Connection<MainDB>,
    params: LocateByLocParams,
) -> Result<Json<Station>, String> {
    match database::locate_chain_by_loc(
        &mut db,
        &params.chain_hash,
        &params.name,
        params.pos,
        params.lat,
        params.lon,
    )
    .await
    {
        Ok(Some(station)) => Ok(Json(station)),
        Ok(None) => Err(String::from("failed to create station")),
        Err(err) => Err(format!("{}", err)),
    }
}
