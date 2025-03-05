use crate::database::MainDB;
use inv_zastavky_core::suggest::chain::{ChainStationsSuggestion, chain_options};
use rocket::get;
use rocket::serde::json::Json;
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
