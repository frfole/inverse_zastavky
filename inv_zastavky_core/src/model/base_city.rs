use serde::Serialize;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Sqlite, query};

#[derive(Serialize, Debug)]
pub struct BaseCity {
    name: String,
    lat: f64,
    lon: f64,
}

impl BaseCity {
    pub fn new(name: String, lat: f64, lon: f64) -> BaseCity {
        BaseCity { name, lat, lon }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn lat(&self) -> f64 {
        self.lat
    }

    pub fn lon(&self) -> f64 {
        self.lon
    }

    pub async fn search(
        db: &mut PoolConnection<Sqlite>,
        search: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let rows: Vec<SqliteRow> = query(
            "SELECT city_name, lat, lon FROM sl_base_cities WHERE city_name LIKE $1 ORDER BY city_name LIMIT 50;")
            .bind(search)
            .fetch_all(&mut **db).await?;
        let mut cities = Vec::new();
        for row in rows {
            let name = row.try_get(0)?;
            let lat = row.try_get(1)?;
            let lon = row.try_get(2)?;
            cities.push(Self::new(name, lat, lon));
        }
        Ok(cities)
    }

    pub async fn get_by_name(
        db: &mut PoolConnection<Sqlite>,
        name: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let rows: Vec<SqliteRow> =
            query("SELECT city_name, lat, lon FROM sl_base_cities WHERE city_name = $1;")
                .bind(name)
                .fetch_all(&mut **db)
                .await?;

        let mut cities = Vec::new();
        for row in rows {
            let name = row.try_get(0)?;
            let lat = row.try_get(1)?;
            let lon = row.try_get(2)?;
            cities.push(Self::new(name, lat, lon));
        }
        Ok(cities)
    }
}
