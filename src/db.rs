use rusqlite::{Connection, Result};

struct Connections {
    id_city_1: i64,
    id_city_2: i64,
    distance: f64,
}

#[derive(Clone)]
pub struct CityDB {
    pub data: Vec<f64>,
    pub coordenadas: Vec<(f64, f64)>,
    pub distancias_tsp: Vec<f64>,
    pub tsp: Vec<i32>,
}

impl CityDB {
    pub fn new(indices_tsp: &Vec<i64>) -> Self {
        let tsp = Self::cargar_tsp(indices_tsp);
        
        CityDB{
            data: vec![-1.0; 1093*1093], 
            coordenadas: vec![(0.0,0.0); 1093] ,
            distancias_tsp: Vec::new(),
            tsp,
        }
    }

    fn cargar_tsp (indices_tsp: &Vec<i64>) -> Vec<i32> {
        let mut tsp = vec![0; 1093];
        for elemento in indices_tsp.iter() {
            tsp[*elemento as usize] = 1;
        }
        return tsp;
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
            self.data[(c.id_city_1*1093 + c.id_city_2) as usize] = c.distance;
            self.data[(c.id_city_2*1093 + c.id_city_1) as usize] = c.distance; // Es dirigida?
            if self.tsp[c.id_city_1 as usize] == 1 && self.tsp[c.id_city_2 as usize] == 1 {
                self.distancias_tsp.push(c.distance);
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

        self.distancias_tsp.sort_by(|a, b| a.partial_cmp(b).unwrap());

        
        Ok(())
    }
    
    pub fn get_latitude_longitude(&mut self, u: i64) -> (f64, f64) {
        return self.coordenadas[u as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use crate::fs;


    fn generar_numeros() -> Vec<i64>{
        let contenido = fs::read_to_string("inputs/input-40.tsp".to_string());

        let numeros: Vec<i64> = contenido.expect("No es un entero").trim().split(',').map(|s| s.parse::<i64>().expect("Error al convertir el numero")).collect();
        return numeros;
    }
    
    #[test]
    fn constructor_city() {
        let cities = CityDB::new(&generar_numeros());
        let mut rng = rand::rng();

        let r: usize = rng.random_range(0..cities.data.len());
        
        assert_eq!(cities.data[0], -1.0);
        assert_eq!(cities.data[1092*1093], -1.0);
        assert_eq!(cities.data[r], -1.0);
        assert_eq!(cities.coordenadas[0], (0.0,0.0));
     }

    #[test]
    fn ok_cargar_datos() {
        let mut cities = CityDB::new(&generar_numeros());
        let _ = cities.cargar_datos();

        let id1 = 1071;
        let latitude1 = 29.30780000000000029;
        let longitude1 = 30.83999999999999986;
        assert_eq!(cities.coordenadas[id1], (latitude1, longitude1));
        let id = 1055;
        let latitude = -3.799999999999999823;
        let longitude = 102.266999999999996;
        assert_eq!(cities.coordenadas[id], (latitude, longitude));
        assert_eq!(cities.data[id1*1093 + id], -1.0);
        let id2 = 1085;
        assert_eq!(cities.data[id1*1093 + id2], 1347317.290000000037);
        
    }
    
    #[test]
    fn ok_get_latitude_longitude(){
        let mut cities = CityDB::new(&generar_numeros());
        let _ = cities.cargar_datos();
        
        let id = 1071;
        let latitude = 29.30780000000000029;
        let longitude = 30.83999999999999986;
        assert_eq!(cities.get_latitude_longitude(id), (latitude, longitude));

        let id = 1055;
        let latitude = -3.799999999999999823;
        let longitude = 102.266999999999996;
        assert_eq!(cities.get_latitude_longitude(id), (latitude, longitude));
    }
}
