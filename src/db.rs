use rusqlite::{Connection, Result};


/// Estructura que almacena las aristas de la grafica.
///
/// Esta estructura es la encargada de almacenar las aristas dentro de la gráfica que leimos en la base de datos
/// De tal manera que:
///
/// - id_city_1 almacena la información corrrespondiente a la primer ciudad.
/// - id_city_2 almacena la información correspondiente a la segunda ciudad.
/// - distance: almacenamos la distancia que se encuentra entre las dos ciudades.
///
/// # Example
/// ```
/// let conection:Connections {
///     id_city_1: 1,
///     id_city_2: 2,
///     distance: 55555.0
/// }
/// ```

struct Connections {
    id_city_1: i64,
    id_city_2: i64,
    distance: f64,
}


/// Estructura correspondiente a la información obtenida de la base de datos.
///
/// - Esta estructura recopila la información correspondiente a los datos de la ciudad
/// - data: Es un vector que contiene los datos de las ciudades
/// - coordenadas: Es un vector de tuplas, los cuales corresponden a las coordenadas de cada ciudad.
/// - distancias_tsp: Es un vector encargado de almacenar todas las distancias que concuerdan con los indices de tsp
/// - tsp: Es un vector encargado de almacer si la ciudad el indice i se encuentra en nuestro tsp
///
/// # Example
/// ```
/// let city = CityDB{
///            data: vec![-1.0; 1093*1093], 
///            coordenadas: vec![(0.0,0.0); 1093] ,
///            distancias_tsp: Vec::new(),
///            tsp,
///        }
/// ```
///

#[derive(Clone)]
pub struct CityDB {
    pub data: Vec<f64>,
    pub coordenadas: Vec<(f64, f64)>,
    pub distancias_tsp: Vec<f64>,
    pub tsp: Vec<i32>,
}

impl CityDB {


    /// Constructor de la Estructura
    ///
    /// Recibe un vector con la información correspondiente a los indices de las ciudades.
    ///
    /// 1. Crea e inicializa la constante tsp de tal manera que será iniciaizada con la información
    /// correspondiente de cargar los indices_tsp (Con un algoritmo bucketSort)
    /// 2. Creamos la estructura CityDB de tal manera que inicializaremos los vectores con el tamaño maximo
    /// de las ciudades que se encuentran en la base de datos, que como sabemos corresponden a 1092.
    ///
    /// # Example
    /// ```
    /// let ciudad = CityDB::new(vectorTsp);
    /// ```
    pub fn new(indices_tsp: &Vec<i64>) -> Self {
        let tsp = Self::cargar_tsp(indices_tsp);
        
        CityDB{
            data: vec![-1.0; 1093*1093], 
            coordenadas: vec![(0.0,0.0); 1093] ,
            distancias_tsp: Vec::new(),
            tsp,
        }
    }


    /// Funcion encargada de cargar el tsp con un algoritmo bucketsort
    ///
    /// Recibe un vector con la información correspondiente a los indices de las ciudades.
    ///
    /// Realiza un algoritmo similar a bucketSort de tal manera que dado el indice de la ciudad marcaremos
    /// la ciudad con un 1, de esta manera sabremos en el vector tsp que ciudades se encuentran en nuestro camino
    ///
    fn cargar_tsp (indices_tsp: &Vec<i64>) -> Vec<i32> {
        let mut tsp = vec![0; 1093];
        for elemento in indices_tsp.iter() {
            tsp[*elemento as usize] = 1;
        }
        return tsp;
    }

    /// Funcion encargada de cargar los datos de la base de datos.
    ///
    /// La funcion se encarga de cargar los datos correspondientes a la base de datos de nuestro problema
    /// y con esta información completar los datos de la estructura.
    ///
    /// 1. Generamos la conexión a la base de datos. 
    /// 2. Obtenemos la información correspondiente a las conexiones entre ciudades y les generamos una estructura Connections
    /// 3. Para cada conexion leida, realizamos lo siguiente:
    ///    * Usaremos el polinomio de direccionamiento de tal manera que podamos simular una matriz. Es de esta manera que agregar la distancia de manera bidireccional a nuestro vector de adyacencias.
    ///    * Revisamos si ambas ciudades se encuentran en nuestro recorrido, en caso de que lo esten, agregamos la distancia a nuestro vector de distancias.
    /// 4. Despues de esto, para cada ciudad obtendremos los valores correspondientes a su latitud y su longitud, los cuales serán almacenados como una tupla en nuestro vector de coordenadas.
    /// 5. Seguido a esto, ordenamos el vector de distancias-
    /// 6. Regresamos que el proceso se ha completado satisfactoriamente.
    ///
    /// # Example
    /// ```
    /// let _ = ciudades.cargar_datos();
    ///```
    
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

    /// Obtener latitude y longitude de una ciudad
    ///
    /// Funcion encargada de obtener la latitude y la longitud de una ciudad en forma de tupla dado el indice 
    /// de la ciudad.
    ///
    /// # Example
    /// ```
    /// let a = ciudades.get_latitude_longitude(1);
    /// ```
    
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
