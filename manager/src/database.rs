use sqlx::{Pool, Sqlite};

pub async fn ensure_tables(db_pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    sqlx::query(
        "
create table if not exists sl_chains(
    chain_hash TEXT,
    station_name TEXT,
    pos integer,
    UNIQUE(chain_hash, station_name, pos)
);
create table if not exists sl_base_stations(
    lat float,
    lon float,
    station_name TEXT
);
create table if not exists sl_base_cities(
    city_name text,
    lat float,
    lon float
);
create table if not exists hl_stations(
    chain_hash TEXT,
    station_name TEXT,
    pos integer,
    stop_id integer,
    UNIQUE(chain_hash, station_name, pos)
);
create table if not exists el_station_names(
    stop_id integer,
    station_name TEXT,
    UNIQUE(stop_id, station_name)
);
create table if not exists el_station_pos(
    stop_id integer UNIQUE,
    lat float,
    lon float
);
CREATE INDEX IF NOT EXISTS sl_chains_idx1 ON sl_chains (chain_hash, station_name);
CREATE INDEX IF NOT EXISTS sl_base_cities_idx1 ON sl_base_cities (city_name);
CREATE INDEX IF NOT EXISTS hl_stations_idx1 ON hl_stations (chain_hash, station_name);
CREATE INDEX IF NOT EXISTS el_station_pos_idx1 ON el_station_pos (stop_id);
CREATE INDEX IF NOT EXISTS el_station_names_idx1 ON el_station_names (stop_id);
",
    )
    .execute(db_pool)
    .await?;
    Ok(())
}
