use crate::App;
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, JsonObject, JsonValue, Value};
use sqlx::sqlite::SqliteRow;
use sqlx::{query, Pool, Row, Sqlite};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

type StopId = i32;

struct Station {
    names: Vec<String>,
    lat: f64,
    lon: f64,
}

impl App {
    pub async fn export(db_pool: &Pool<Sqlite>, output_path: PathBuf) -> anyhow::Result<()> {
        let rows: Vec<SqliteRow> = query("SELECT stop_id, lat, lon, station_name FROM el_station_names JOIN el_station_pos USING (stop_id)")
            .fetch_all(db_pool)
            .await?;

        let mut stations_names: HashMap<StopId, Vec<String>> = HashMap::new();
        let mut stations_locs: HashMap<StopId, (f64, f64)> = HashMap::new();
        for row in rows {
            let stop_id: StopId = row.try_get(0)?;
            if !stations_names.contains_key(&stop_id) {
                stations_locs.insert(stop_id, (row.try_get(1)?, row.try_get(2)?));
                stations_names.insert(stop_id, Vec::new());
            }
            stations_names
                .get_mut(&stop_id)
                .unwrap()
                .push(row.try_get(3)?);
        }
        let mut stations = Vec::new();
        for (stop_id, names) in stations_names {
            stations.push(Station {
                names,
                lat: stations_locs[&stop_id].0,
                lon: stations_locs[&stop_id].1,
            });
        }

        let collection = GeoJson::FeatureCollection(FeatureCollection {
            bbox: None,
            features: stations
                .iter()
                .map(|station| {
                    let geometry = Geometry::new(Value::Point(vec![station.lon, station.lat]));
                    let mut properties = JsonObject::new();
                    properties.insert(String::from("name"), JsonValue::from(station.names.clone()));
                    return Feature {
                        bbox: None,
                        geometry: Some(geometry),
                        id: None,
                        properties: Some(properties),
                        foreign_members: None,
                    };
                })
                .collect::<Vec<_>>(),
            foreign_members: None,
        });
        let mut file = File::create(output_path)?;
        file.write_all(collection.to_string().as_bytes())?;
        Ok(())
    }
}
