use crate::database::MainDB;
use crate::model::{BBox, Station, StopId};
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

#[derive(FromForm)]
pub struct MoveStationParams {
    stop_id: StopId,
    lat: f64,
    lon: f64,
}

#[derive(FromForm)]
pub struct AddStationParams {
    lat: f64,
    lon: f64,
    name: String,
}

#[derive(FromForm)]
pub struct RmStationParams {
    stop_id: StopId,
}

#[derive(FromForm)]
pub struct RmStationNameParams {
    stop_id: StopId,
    name: String,
}

#[derive(FromForm)]
pub struct AddStationNameParams {
    stop_id: StopId,
    name: String,
}

impl Into<BBox> for BBoxParams {
    fn into(self) -> BBox {
        BBox::new([self.lat_from, self.lat_to], [self.lon_from, self.lon_to])
    }
}

#[get("/el_stations_bbox?<bbox..>")]
pub async fn get_stations_by_bbox(
    mut db: Connection<MainDB>,
    bbox: BBoxParams,
) -> Result<Json<Vec<Station>>, String> {
    let stations = Station::get_by_bbox(&mut db, &bbox.into()).await;
    match stations {
        Ok(stations) => Ok(Json(stations)),
        Err(err) => Err(format!("{}", err)),
    }
}

#[get("/el_add_station?<params..>")]
pub async fn add_station(
    mut db: Connection<MainDB>,
    params: AddStationParams,
) -> Result<Json<Station>, String> {
    let station = Station::create_station(
        &mut db,
        rand::random::<i32>() as StopId,
        params.lat,
        params.lon,
        &params.name,
    )
    .await;
    match station {
        Ok(Some(station)) => Ok(Json(station)),
        Ok(None) => Err(String::from("no station added")),
        Err(err) => Err(format!("{}", err)),
    }
}

#[get("/el_rm_station?<params..>")]
pub async fn rm_station(mut db: Connection<MainDB>, params: RmStationParams) -> Result<(), String> {
    let station = Station::remove_station(&mut db, params.stop_id).await;
    match station {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }
}

#[get("/el_move_station?<params..>")]
pub async fn move_station(
    mut db: Connection<MainDB>,
    params: MoveStationParams,
) -> Result<Json<Station>, String> {
    let station = Station::move_station(&mut db, params.stop_id, params.lat, params.lon).await;
    match station {
        Ok(Some(station)) => Ok(Json(station)),
        Ok(None) => Err(String::from("no station to move")),
        Err(err) => Err(format!("{}", err)),
    }
}

#[get("/el_add_station_name?<params..>")]
pub async fn add_station_name(
    mut db: Connection<MainDB>,
    params: AddStationNameParams,
) -> Result<Json<Station>, String> {
    let station = Station::add_name(&mut db, params.stop_id, &params.name).await;
    match station {
        Ok(Some(station)) => Ok(Json(station)),
        Ok(None) => Err(String::from("no station updated")),
        Err(err) => Err(format!("{}", err)),
    }
}

#[get("/el_rm_station_name?<params..>")]
pub async fn rm_station_name(
    mut db: Connection<MainDB>,
    params: RmStationNameParams,
) -> Result<Json<Station>, String> {
    let station = Station::remove_station_name(&mut db, params.stop_id, &params.name).await;
    match station {
        Ok(Some(station)) => Ok(Json(station)),
        Ok(None) => Err(String::from("no station updated")),
        Err(err) => Err(format!("{}", err)),
    }
}
