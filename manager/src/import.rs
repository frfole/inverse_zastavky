use crate::{netex, App};
use base64::Engine;
use md5::{Digest, Md5};
use sqlx::{Database, Pool, QueryBuilder, Sqlite};
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, Read};
use std::path::PathBuf;

impl App {
    pub async fn import(db_pool: &Pool<Sqlite>, netex_path: Option<PathBuf>) -> anyhow::Result<()> {
        let netex_path = netex_path.unwrap();
        let netex_file = File::open(netex_path)?;
        let mut archive = zip::ZipArchive::new(netex_file)?;

        let mut station_chains = Vec::new();
        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            let buf_reader = BufReader::new(file);
            let reader = quick_xml::Reader::from_reader(buf_reader);
            station_chains.extend(netex::parse::parse_netex(reader)?);
            println!("{}/{}", i, archive.len());
        }
        println!("{}", station_chains.len());

        let mut stations = HashMap::new();
        for chain in station_chains {
            let name = chain.join("|");
            let hash = Md5::digest(name);
            stations.insert(base64::prelude::BASE64_STANDARD.encode(hash), chain);
        }
        println!("{}", stations.len());

        sqlx::query("DELETE FROM sl_chains;")
            .execute(db_pool)
            .await?;
        let stations: Vec<_> = stations
            .iter()
            .fold(Vec::new(), |mut acc, (hash, stations)| {
                acc.extend(
                    stations
                        .iter()
                        .enumerate()
                        .map(|(i, station)| (hash, station, i)),
                );
                acc
            });

        for stations in stations.chunks(1000) {
            let mut builder =
                QueryBuilder::new("INSERT INTO sl_chains(chain_hash, station_name, pos)");
            builder.push_values(stations, |mut b, station| {
                b.push_bind(station.0)
                    .push_bind(station.1)
                    .push_bind(station.2 as i32);
            });
            builder.build().execute(db_pool).await?;
        }
        return Ok(());
    }
}
