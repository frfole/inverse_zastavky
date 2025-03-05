use crate::database::MainDB;
use inv_zastavky_core::model::base_city::BaseCity;
use inv_zastavky_core::model::base_station::BaseStation;
use inv_zastavky_core::model::bbox::BBox;
use rocket::serde::json::Json;
use rocket::{get, FromForm};
use rocket_db_pools::Connection;

#[derive(FromForm)]
pub struct BBoxParams {
    lat_from: f64,
    lat_to: f64,
    lon_from: f64,
    lon_to: f64,
}

impl Into<BBox> for BBoxParams {
    fn into(self) -> BBox {
        BBox::new([self.lat_from, self.lat_to], [self.lon_from, self.lon_to])
    }
}

#[get("/sl_base_stations?<bbox..>")]
pub async fn get_base_stations_by_bbox(
    mut db: Connection<MainDB>,
    bbox: BBoxParams,
) -> Result<Json<Vec<BaseStation>>, String> {
    let stations = BaseStation::get_by_bbox(&mut db, &bbox.into()).await;
    match stations {
        Ok(stations) => Ok(Json(stations)),
        Err(err) => Err(format!("{}", err)),
    }
}

#[get("/sl_base_city?<query>")]
pub async fn search_base_city(
    mut db: Connection<MainDB>,
    query: String,
) -> Result<Json<Vec<BaseCity>>, String> {
    let cities = BaseCity::search(&mut db, &query).await;
    match cities {
        Ok(cities) => Ok(Json(cities)),
        Err(err) => Err(format!("{}", err)),
    }
}
