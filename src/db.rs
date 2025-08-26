use rusqlite::{Connection, Result};
use rusqlite::params;
use std::boxed::Box;


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
    pub data: Box<[[f64; 1092]; 1092]>,
    pub coordenadas: Box<[(f64,f64); 1092]>,
    pub distanciaMaxima: f64,
}

impl CityDB {
    pub fn new() -> Self {
        let data = Box::new([[-1.0; 1092]; 1092]);
        CityDB{
            data,
            coordenadas: Box::new([(0.0,0.0); 1092]) ,
            distanciaMaxima: 0.0,
        }
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

        stmt = conn.prepare("SELECT id, latitude, longitude FROM cities")?;
        let cities_iter = stmt.query_map([], |row| {
            Ok((row.get::<_,i64>(0)?, row.get::<_,f64>(1)?, row.get::<_,f64>(2)?))
        })?;
        
        for city in cities_iter {
            let (id, lat, lon) = city?;
            self.coordenadas[id as usize] = (lat, lon);
        }
        
        Ok(())
    }

    pub fn get_latitude_longitude(&mut self, u: i64) -> (f64, f64) {
        return self.coordenadas[u as usize];
    }
}
