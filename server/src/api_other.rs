use crate::database::MainDB;
use crate::model::Stats;
use rocket::get;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

#[get("/other_stats")]
pub async fn other_stats(mut db: Connection<MainDB>) -> Result<Json<Stats>, String> {
    let cities = Stats::get(&mut db).await;
    match cities {
        Ok(stats) => Ok(Json(stats)),
        Err(err) => Err(format!("{}", err)),
    }
}
