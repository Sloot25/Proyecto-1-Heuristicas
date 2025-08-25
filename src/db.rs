use rusqlite::{Connection, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Cities {
    id: i64,
    name: String,
    country: String,
    population: i64,
    latitude: f64,
    longitude: f64,
}

struct Connections {
    id_city_1: i64,
    id_city_2: i64,
    distance: f64,
}

pub struct CityDB {
   pub data: HashMap<i64, Cities>,
}

impl CityDB {
    pub fn new() -> Self {
        CityDB{ data: HashMap::new()}
    }

    pub fn cargar_datos(&mut self) -> Result<()> {
        let conn = Connection::open("tsp.db")?;
        let mut stmt = conn.prepare("SELECT * FROM cities")?;
        let cities_iter = stmt.query_map([], |row| {
            Ok(Cities {
                id: row.get(0)?,
                name: row.get(1)?,
                country: row.get(2)?,
                population: row.get(3)?,
                latitude: row.get(4)?,
                longitude: row.get(5)?,
            })
        })?;

        for city in cities_iter {
            let c = city?;
            self.data.insert(c.id,c);
        }

        stmt = conn.prepare("SELECT * FROM connections")?;
        let connections_iter = stmt.query_map([], |row| {
            Ok(Connections {
                id_city_1: row.get(0)?,
                id_city_2: row.get(1)?,
                distance: row.get(2)?,
            })
        })?;

        for connect in connections_iter {
            let c = connect?;
            self.connections.insert()
        }

        Ok(())
    }

    pub fn getDistanciaNatural(&mut self,u: i64, v: i64) {
        
    }

    pub fn peso(&mut self, u: i64, v: i64) {
        
    }

    pub fn getDistanciaMaxima(&mut self, u: i64, v: i64) {
        
    }
    
}
