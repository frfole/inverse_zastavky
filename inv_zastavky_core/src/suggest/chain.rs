use crate::model::StopId;
use crate::model::base_city::BaseCity;
use crate::model::chain_station::ChainStation;
use crate::model::station::Station;
use crate::utils::geo::{approx_distance, approx_len};
use serde::Serialize;
use sqlx::Sqlite;
use sqlx::pool::PoolConnection;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ChainStationsSuggestion {
    len: f64,
    chain_hash: String,
    path: Vec<Option<(f64, f64, StopId)>>,
}

#[derive(Debug, Serialize)]
pub struct ChainCitySuggestion {
    len: f64,
    chain_hash: String,
    path: Vec<(f64, f64)>,
}

pub async fn path_options(
    db_pool: &mut PoolConnection<Sqlite>,
    chain_hash: &str,
    city_remap: &HashMap<String, String>,
) -> anyhow::Result<Vec<ChainCitySuggestion>> {
    // get names of cities, if they are part of the name
    let mut city_chain = Vec::new();
    for station in ChainStation::get_by_chain_hash(db_pool, chain_hash).await? {
        let city_name =
            if let Some(city_name) = station.name().split_once(" [").map(|(left, _)| left) {
                city_name.to_string()
            } else if let Some(city_name) = station.name().split_once(",").map(|(left, _)| left) {
                city_name.to_string()
            } else {
                station.name().to_string()
            };
        if let Some(city_name) = city_remap.get(&city_name) {
            city_chain.push(city_name.clone());
        } else {
            city_chain.push(city_name);
        }
    }

    // get position of cities
    let mut cities = HashMap::new();
    for chain_city in &city_chain {
        let mut cities_pos: Vec<BaseCity> = Vec::new();
        for city_pos in BaseCity::get_by_name(db_pool, chain_city.as_str()).await? {
            if !cities_pos.iter().any(|other| {
                approx_distance(other.lat(), other.lon(), city_pos.lat(), city_pos.lon()) < 0.5
            }) {
                cities_pos.push(city_pos);
            }
        }
        cities.insert(chain_city.to_string(), cities_pos);
    }

    let mut paths: Vec<Vec<(f64, f64)>> = Vec::new();
    // try to get first city
    if let Some(possible_cities) = cities.get(&city_chain[0]) {
        let mut new_paths = Vec::new();
        for city in possible_cities {
            let mut new_path = Vec::new();
            new_path.push((city.lat(), city.lon()));
            new_paths.push(new_path);
        }
        paths = new_paths;
    }

    for window in city_chain.windows(2) {
        let prev = &window[0];
        let cur = &window[1];
        if prev == cur {
            continue;
        }
        if let Some(possible_cities) = cities.get(cur.as_str()) {
            let mut new_paths = Vec::new();
            if paths.is_empty() {
                for city in possible_cities {
                    let mut new_path = Vec::new();
                    new_path.push((city.lat(), city.lon()));
                    new_paths.push(new_path);
                }
            } else {
                for path in paths {
                    for city in possible_cities {
                        let mut new_path = path.clone();
                        new_path.push((city.lat(), city.lon()));
                        new_paths.push(new_path);
                    }
                }
            }
            paths = new_paths;
        }
        // try to prevent OOM
        if paths.len() > 100_000 {
            break;
        }
    }

    let paths = paths
        .iter()
        .map(|path| ChainCitySuggestion {
            len: approx_len(&path.iter().map(|(lat, lon)| (*lat, *lon)).collect()),
            chain_hash: chain_hash.to_string(),
            path: path.to_owned(),
        })
        .collect::<Vec<_>>();

    Ok(paths)
}

pub async fn chain_options(
    db_pool: &mut PoolConnection<Sqlite>,
    chain_hash: &str,
) -> anyhow::Result<Vec<ChainStationsSuggestion>> {
    let station_chain = ChainStation::get_by_chain_hash(db_pool, chain_hash).await?;

    // get position of stations
    let mut stations = HashMap::new();
    for chain_station in &station_chain {
        stations.insert(
            chain_station.name().to_string(),
            Station::get_by_name(db_pool, chain_station.name()).await?,
        );
    }

    let mut paths: Vec<Vec<Option<(f64, f64, StopId)>>> = Vec::new();
    let mut chain_station_iter = station_chain.iter();
    // try to get first city
    if let Some(chain_station) = chain_station_iter.next() {
        if let Some(possible_stations) = stations.get(chain_station.name()) {
            for station in possible_stations {
                let mut new_path = Vec::new();
                new_path.push(Some((station.lat(), station.lon(), station.stop_id())));
                paths.push(new_path);
            }
            if possible_stations.is_empty() {
                let mut new_path = Vec::new();
                new_path.push(None);
                paths.push(new_path);
            }
        } else {
            let mut new_path = Vec::new();
            new_path.push(None);
            paths.push(new_path);
        }
    }

    for chain_station in chain_station_iter {
        if let Some(possible_stations) = stations.get(chain_station.name()) {
            let mut new_paths = Vec::new();
            for path in paths {
                for station in possible_stations {
                    let mut new_path = path.clone();
                    new_path.push(Some((station.lat(), station.lon(), station.stop_id())));
                    new_paths.push(new_path);
                }
                if possible_stations.is_empty() {
                    let mut new_path = path.clone();
                    new_path.push(None);
                    new_paths.push(new_path);
                }
            }
            paths = new_paths;
        } else {
            let mut new_paths = Vec::new();
            for path in paths {
                let mut new_path = path.clone();
                new_path.push(None);
                new_paths.push(new_path);
            }
            paths = new_paths;
        }
    }

    let paths = paths
        .iter()
        .map(|path| ChainStationsSuggestion {
            len: approx_len(
                &path
                    .iter()
                    .filter_map(|a| *a)
                    .map(|(lat, lon, _)| (lat, lon))
                    .collect(),
            ),
            chain_hash: chain_hash.to_string(),
            path: path.to_owned(),
        })
        .collect::<Vec<_>>();

    Ok(paths)
}
