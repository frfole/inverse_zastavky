use sqlx::{Database, Pool, Sqlite};

pub async fn ensure_tables(db_pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    sqlx::query(
        "
create table if not exists sl_chains(
    chain_hash TEXT,
    station_name TEXT,
    pos integer,
    UNIQUE(chain_hash, station_name, pos)
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
    lon float,
    );
",
    )
    .execute(db_pool)
    .await?;
    Ok(())
}
