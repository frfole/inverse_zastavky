use std::collections::HashMap;

pub struct ServerConfig {
    pub city_remap: HashMap<String, String>,
}

impl ServerConfig {
    pub fn new() -> ServerConfig {
        let mut map = HashMap::new();
        for line in include_str!("czech-city-remap.txt").lines() {
            if let Some((left, right)) = line.split_once("\t") {
                map.insert(String::from(left), String::from(right));
            }
        }
        ServerConfig { city_remap: map }
    }
}
