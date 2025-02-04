use rocket::serde::Serialize;

pub type StopId = i32;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Station {
    stop_id: StopId,
    names: Vec<String>,
    lat: f64,
    lon: f64,
}

pub struct BBox {
    lat_from: f64,
    lat_to: f64,
    lon_from: f64,
    lon_to: f64,
}

impl Station {
    pub fn new(stop_id: StopId, names: Vec<String>, lat: f64, lon: f64) -> Station {
        Station {
            stop_id,
            names,
            lat,
            lon,
        }
    }
}

impl BBox {
    pub fn new(lat: [f64; 2], lon: [f64; 2]) -> BBox {
        BBox {
            lat_from: f64::min(lat[0], lat[1]),
            lat_to: f64::max(lat[0], lat[1]),
            lon_from: f64::min(lon[0], lon[1]),
            lon_to: f64::max(lon[0], lon[1]),
        }
    }

    pub fn lat_from(&self) -> f64 {
        self.lat_from
    }

    pub fn lat_to(&self) -> f64 {
        self.lat_to
    }

    pub fn lon_from(&self) -> f64 {
        self.lon_from
    }

    pub fn lon_to(&self) -> f64 {
        self.lon_to
    }
}
