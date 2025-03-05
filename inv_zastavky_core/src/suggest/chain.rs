use crate::model::StopId;
use crate::model::base_city::BaseCity;
use crate::model::chain_station::ChainStation;
use crate::model::station::Station;
use crate::utils::geo::approx_len;
use serde::Serialize;
use sqlx::Sqlite;
use sqlx::pool::PoolConnection;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ChainStationsSuggestion {
    len: f64,
    path: Vec<Option<(f64, f64, StopId)>>,
}

#[derive(Debug, Serialize)]
pub struct ChainCitySuggestion {
    len: f64,
    path: Vec<(f64, f64)>,
}

pub async fn path_options(
    db_pool: &mut PoolConnection<Sqlite>,
    chain_hash: &str,
) -> anyhow::Result<Vec<ChainCitySuggestion>> {
    // get names of cities, if they are part of the name
    let mut city_chain = Vec::new();
    for station in ChainStation::get_by_chain_hash(db_pool, chain_hash).await? {
        if let Some(city_name) = station.name().split_once(" [").map(|(left, _)| left) {
            city_chain.push(city_name.to_string());
        } else if let Some(city_name) = station.name().split_once(",").map(|(left, _)| left) {
            city_chain.push(city_name.to_string());
        } else {
            city_chain.push(station.name().to_string());
        }
    }

    // get position of cities
    let mut cities = HashMap::new();
    for chain_city in &city_chain {
        cities.insert(
            chain_city.to_string(),
            BaseCity::get_by_name(db_pool, chain_city.as_str()).await?,
        );
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
    }

    let paths = paths
        .iter()
        .map(|path| ChainCitySuggestion {
            len: approx_len(&path.iter().map(|(lat, lon)| (*lat, *lon)).collect()),
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

    let mut paths: Vec<Vec<Option<(f64, f64, StopId)>>>;
    // try to get first city
    if let Some(possible_stations) = stations.get(station_chain[0].name()) {
        let mut new_paths = Vec::new();
        for station in possible_stations {
            let mut new_path = Vec::new();
            new_path.push(Some((station.lat(), station.lon(), station.stop_id())));
            new_paths.push(new_path);
        }
        if possible_stations.is_empty() {
            let mut new_path = Vec::new();
            new_path.push(None);
            new_paths.push(new_path);
        }
        paths = new_paths;
    } else {
        let mut new_paths = Vec::new();
        let mut new_path = Vec::new();
        new_path.push(None);
        new_paths.push(new_path);
        paths = new_paths;
    }

    // TODO: replace with iterator
    for window in station_chain.windows(2) {
        let cur = window[1].name();
        if let Some(possible_stations) = stations.get(cur) {
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
            path: path.to_owned(),
        })
        .collect::<Vec<_>>();

    Ok(paths)
}
