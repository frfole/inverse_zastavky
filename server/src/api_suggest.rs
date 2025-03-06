use crate::config::ServerConfig;
use crate::database::MainDB;
use inv_zastavky_core::suggest::chain::{
    ChainCitySuggestion, ChainStationsSuggestion, chain_options, path_options,
};
use rocket::serde::json::Json;
use rocket::{State, get};
use rocket_db_pools::Connection;

#[get("/suggest_stations?<chain_hash>")]
pub async fn suggest_stations(
    mut db: Connection<MainDB>,
    chain_hash: String,
) -> Result<Json<Vec<ChainStationsSuggestion>>, String> {
    let suggestion = chain_options(&mut db, &chain_hash).await;
    match suggestion {
        Ok(cities) => Ok(Json(cities)),
        Err(err) => Err(format!("{}", err)),
    }
}

#[get("/suggest_cities?<chain_hash>")]
pub async fn suggest_cities(
    mut db: Connection<MainDB>,
    state: &State<ServerConfig>,
    chain_hash: String,
) -> Result<Json<Vec<ChainCitySuggestion>>, String> {
    let suggestion = path_options(&mut db, &chain_hash, &state.city_remap).await;
    match suggestion {
        Ok(cities) => Ok(Json(cities)),
        Err(err) => Err(format!("{}", err)),
    }
}
