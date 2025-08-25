use rusqlite::{Connection, Result};
use rusqlite::params;

#[derive(Debug)]
pub struct Cities {
    id: i64,
    name: String,
    country: String,
    population: i64,
    latitude: f64,
    longitude: f64,
}
impl Cities {
    pub fn new() -> Self{
        Cities {
            id: 0,
            name: String::new(),
            country: String::new(),
            population: 0,
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}
struct Connections {
    id_city_1: i64,
    id_city_2: i64,
    distance: f64,
}

pub struct CityDB {
    pub data: [[f64; 1092]; 1092],
    pub distanciaMaxima: f64,
}

impl CityDB {
    pub fn new() -> Self {
        CityDB{ data: [[-1.0;1092];1092], distanciaMaxima: 0.0, }
    }

    pub fn cargar_datos(&mut self) -> Result<()> {
        let conn = Connection::open("tsp.db")?;

        let mut stmt = conn.prepare("SELECT * FROM connections")?;
        let connections_iter = stmt.query_map([], |row| {
            Ok(Connections {
                id_city_1: row.get(0)?,
                id_city_2: row.get(1)?,
                distance: row.get(2)?,
            })
        })?;

        for connect in connections_iter {
            let c = connect?;
            self.data[c.id_city_1 as usize][c.id_city_2 as usize] = c.distance;
            self.data[c.id_city_2 as usize][c.id_city_1 as usize] = c.distance; // Es dirigida?
            if c.distance > self.distanciaMaxima {
                self.distanciaMaxima = c.distance;
            }
        }

        Ok(())
    }

    pub fn get_latitude_longitude(&mut self, u: i64) -> Result<(f64, f64)> {
        let conn = Connection::open("tsp.db")?;

        let mut stmt = conn.prepare("SELECT * FROM cities WHERE id = :id")?;
        let cities_iter = stmt.query_map(params![u], |row| {
            Ok(Cities {
                id: row.get(0)?,
                name: row.get(1)?,
                country: row.get(2)?,
                population: row.get(3)?,
                latitude: row.get(4)?,
                longitude: row.get(5)?,
            })
        })?;
        
        let mut c = Cities::new();
            
        for city in cities_iter{
            c = city?;
        }
        Ok((c.latitude, c.longitude))
    }
}
