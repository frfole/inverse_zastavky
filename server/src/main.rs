use crate::database::MainDB;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::http::Header;
use rocket::{launch, routes, Request, Response};
use rocket_db_pools::Database;

mod api_chain;
mod api_stations;
mod database;
mod model;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .attach(MainDB::init())
        .mount(
            "/api",
            routes![
                api_chain::list_sl_chains,
                api_stations::get_stations_by_bbox,
                api_stations::add_station,
                api_stations::rm_station,
                api_stations::add_station_name,
                api_stations::rm_station_name,
                api_stations::move_station,
            ],
        )
        .mount("/", FileServer::from("web_ui"))
}

struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
