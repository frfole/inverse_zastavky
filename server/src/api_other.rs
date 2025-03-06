use crate::config::ServerConfig;
use crate::database::MainDB;
use inv_zastavky_core::model::stats::Stats;
use rocket::serde::json::Json;
use rocket::{State, get};
use rocket_db_pools::Connection;
use std::collections::HashMap;

#[get("/other_stats")]
pub async fn other_stats(mut db: Connection<MainDB>) -> Result<Json<Stats>, String> {
    let cities = Stats::get(&mut db).await;
    match cities {
        Ok(stats) => Ok(Json(stats)),
        Err(err) => Err(format!("{}", err)),
    }
}

#[get("/other_city_remap")]
pub async fn other_city_remap(config: &State<ServerConfig>) -> Json<&HashMap<String, String>> {
    Json(&config.city_remap)
}
