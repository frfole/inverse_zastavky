A tool for creating a database of location and names of public transport stations in Czechia

Nástroj pro tvorbu databáze poloh zastávek veřejné dopravy v České republice a jejich možných názvů.

## Setup

1. Download [timetables from CIS JŘ](https://portal.cisjr.cz/pub/netex/NeTEx_VerejnaLinkovaDoprava.zip).
2. Import sequences of stations extracted from timetables.
   `cargo run --bin manager --db-url db.sqlite import --netex-path NeTEx_VerejnaLinkovaDoprava.zip`
3. Optionally import locations of stations
   `cargo run --bin manager --db-url db.sqlite import --base-stations base_stations.geojson`
4. Optionally import locations of cities
   `cargo run --bin manager --db-url db.sqlite import --base-cities base_cities.geojson`
5. Build `web_ui` using `npm build`
6. Start server using `cargo run --bin server`

## Usage

In editor mode, you can create stations, modify theirs name and location and remove them.

In locate mode you place or select stations from sequence of stations
