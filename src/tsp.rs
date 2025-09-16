use rand::SeedableRng;
use rand::Rng;
use rand::rngs::StdRng;
use crate::grafica::Grafica;

/// Estructura correspondiente al TSP
///
/// La estructura almacena los valores que consideramos de utilidad para el TSP, entre ellos tenemos:
/// - grafica:Grafica: Es la estructura Grafica que nos servirá para realizar el calculo de las operaciones.
/// - soluciones_aceptadas:`Vec<f64>`: Es el vector que almacena los valores de las soluciones aceptadas.
/// - solucion_actual:`Vec<i64>`: Es el vector que almacena la solucion actual, es decir almacena las ciudades y el orden del TSP.
/// - temperatura:f64: Es la temperatura actual del sistema, valor guardado para moverlo entre las distintas funciones.
/// - promedio:f64: Es el promedio de la solución actual, se almacena para no tener que recalcularlo y moverlo entre funciones
/// - mejor_solucion:f64: Es el peso de la mejor solucion
/// - normalizador: Es el normalizador que nos permite calcular el peso de las soluciones.
/// - random: Es el random que nos permite obtener distintos enteros.
/// - peso_solucion_actual:f64: Es el peso de la solucion actual
/// - mejor_solucion_arr:`Vec<i64>`: Es el vector que almacena la mejor solucion, es decir almacena las ciudades y el orden del TSP.
///
pub struct Tsp {
    grafica: Grafica,
    pub soluciones_aceptadas: Vec<f64>,
    pub solucion_actual: Vec<i64>,
    temperatura: f64,
    promedio: f64,
    pub mejor_solucion: f64,
    normalizador: f64,
    random: StdRng,
    pub peso_solucion_actual: f64,
    pub mejor_solucion_arr: Vec<i64>,
    pub soluciones_para_svg: Vec<(f64, bool)>,
}

impl Tsp {

    /// Constructor de la estructura TSP
    ///
    /// Recibe como parametros lo siguiente:
    /// - temperatura:f64 : Es la temperatura inicial del sistema
    /// - solucion_actual: `Vec<i64>` : Es la solucion de ciudades que tenemos actualmente.
    /// - semilla:i64 : Es la semilla con la cual se va a inicializar nuestro generador de numeros pseudoaleatorios.
    ///
    /// Se encarga de inicializar la estructura, en el que la mayor parte de los valores de la estructura son inicializados como valores nuevos, excepto el rng y el normalizador.
    ///
    pub fn new(temperatura: f64, grafica: Grafica, solucion_actual: Vec<i64>, semilla: i64) -> Self {
        let rng = StdRng::seed_from_u64(semilla as u64);
        let normalizador = Self::get_normalizador(&grafica.db.distancias_tsp, solucion_actual.len());
        Tsp {
            grafica,
            solucion_actual,
            soluciones_aceptadas: Vec::new(),
            temperatura,
            promedio: 0.0,
            mejor_solucion: f64::MAX,
            normalizador,
            random: rng,
            peso_solucion_actual: 0.0,
            mejor_solucion_arr: Vec::new(),
            soluciones_para_svg: Vec::new(),
        }

        
    }

    /// Funcion encargada de calcular un lote de soluciones
    ///
    /// La funcion se encarga de calcular soluciones, sobre las cuales vamos a aceptar las
    /// mejores soluciones hasta completar la cantidad correspondiente a un lote. Dicho esto se almacenara
    /// las soluciones aceptadas y se calculara el promedio de las soluciones.
    ///
    fn calcular_lote(&mut self) {
        let mut c: i64 = 0;
        let mut r: f64 = 0.0;
        let mut i: i64 = 0;
        let l = 7000;
        
        while c < l {
            let a = self.get_vecino();
            let b = self.get_vecino();
            let new_sol = self.intercambiar_ciudades(a as usize, b as usize);
            
            if new_sol < (self.peso_solucion_actual + self.temperatura) {
                c = c+1;
                r = r + new_sol;
                self.soluciones_aceptadas.push(new_sol);
                self.peso_solucion_actual = new_sol;
                let mut es_mejora = false;
                if new_sol < self.mejor_solucion {
                    self.mejor_solucion = new_sol;
                    self.mejor_solucion_arr = self.solucion_actual.clone();
                    es_mejora = true;
                }
                self.soluciones_para_svg.push((new_sol, es_mejora));
            } else {
                self.intercambiar_ciudades(a as usize,b as usize);
            }
            i = i + 1;
        }

        self.promedio = r/(c as f64);
    }

    /// Funcion encargada de calcular el peso de la solución actual.
    ///
    /// La función se encarga de recorrer nuestro vector de soluciones, calculando el peso entre cada par
    /// de ciudades en el recorrido. para después dividirlo entre el normalizador. Calculando de esta forma el peso de la solución.
    ///
    /// # Example
    /// ```
    /// let peso_solucion = tsp.calcular_solucion();
    /// ```
    pub fn calcular_solucion(&mut self) -> f64 {
        let mut i: usize = 0;
        let mut j: usize = 1;
        let mut res: f64 = 0.0;
        
        while j < self.solucion_actual.len()  {
            res = res + self.grafica.peso(self.solucion_actual[i], self.solucion_actual[j]);
            i = i+1;
            j = j+1;
        }

        let s:f64 = res/self.normalizador;
        return s;
    }

    /// Funcion encargada de obtener el normalizador.
    ///
    /// La funcion recibe la lista ordenada de distancias y el tamaño de las ciudades. De esta manera es que obtenemos n maximos valores de la lista, los sumamos y ese será el valor que devolveremos
    ///
    /// Regresa el normalizador en forma f64.
    ///
    fn get_normalizador(lista_ordenada: &Vec<f64>, n: usize) -> f64 {
        let mut i = lista_ordenada.len() - n + 1;
        let mut normalizador: f64 = 0.0;
        while i < lista_ordenada.len(){
            normalizador = normalizador + lista_ordenada[i];
            i = i+1;
        }
        return normalizador;
    }

    /// Funcion encargada de generar la primer solucion.
    ///
    /// Dado un vector de ciudades, lo revuelve de forma aleatoria para así obtener una permutación aleatoria
    /// la cual será el punto de partida de nuestro algoritmo.
    ///
    /// # Example
    /// ```
    /// tsp.generar_primer_solucion();
    /// ```
    pub fn generar_primer_solucion(&mut self) {
        let mut i: i64 = 0;
        while i < (self.solucion_actual.len() as i64) {
            let k: usize = self.random.random_range(0..self.solucion_actual.len());
            self.intercambiar_ciudades(i as usize,k);
            i = i+1;
        }
    }
    
    /// Funcion encargada de intercambiar las ciudades y recalcular el peso.
    ///
    /// Recibe como parametros lo siguiente:
    /// - a:usize : Corresponde al indice de la primer ciudad.
    /// - b:usize : Corresponde al indice de la segunda ciudad.
    ///
    fn intercambiar_ciudades(&mut self, a: usize, b: usize) -> f64{
        let temp = self.solucion_actual[a];
        let mut solucion = self.peso_solucion_actual;

        solucion = solucion * self.normalizador;
        
        if a != 0 && a!= self.solucion_actual.len()-1 {
            solucion = solucion - self.grafica.peso(self.solucion_actual[a-1], self.solucion_actual[a]);
            solucion = solucion -  self.grafica.peso(self.solucion_actual[a], self.solucion_actual[a+1]);
        } else if a != 0 {
            solucion = solucion - self.grafica.peso(self.solucion_actual[a-1], self.solucion_actual[a]);
        } else {
            solucion = solucion -  self.grafica.peso(self.solucion_actual[a], self.solucion_actual[a+1]);
        }

        if b != 0 && b != self.solucion_actual.len()-1 {
            solucion = solucion - self.grafica.peso(self.solucion_actual[b-1], self.solucion_actual[b]);
            solucion = solucion - self.grafica.peso(self.solucion_actual[b], self.solucion_actual[b+1]);
        } else if b != 0 {
            solucion = solucion - self.grafica.peso(self.solucion_actual[b-1], self.solucion_actual[b]);
        } else {
            solucion = solucion - self.grafica.peso(self.solucion_actual[b], self.solucion_actual[b+1]);
        }
        
        self.solucion_actual[a] = self.solucion_actual[b];
        self.solucion_actual[b] = temp;

        if a != 0 && a!= self.solucion_actual.len()-1 {
            solucion = solucion + self.grafica.peso(self.solucion_actual[a-1], self.solucion_actual[a]);
            solucion = solucion +  self.grafica.peso(self.solucion_actual[a], self.solucion_actual[a+1]);
        } else if a != 0 {
            solucion = solucion + self.grafica.peso(self.solucion_actual[a-1], self.solucion_actual[a]);
        } else {
            solucion = solucion +  self.grafica.peso(self.solucion_actual[a], self.solucion_actual[a+1]);
        }

        if b != 0 && b != self.solucion_actual.len()-1 {
            solucion = solucion + self.grafica.peso(self.solucion_actual[b-1], self.solucion_actual[b]);
            solucion = solucion + self.grafica.peso(self.solucion_actual[b], self.solucion_actual[b+1]);
        } else if b != 0 {
            solucion = solucion + self.grafica.peso(self.solucion_actual[b-1], self.solucion_actual[b]);
        } else {
            solucion = solucion + self.grafica.peso(self.solucion_actual[b], self.solucion_actual[b+1]);
        }

        solucion = solucion/self.normalizador;
        return solucion;
    }

    /// Funcion encargada de obtener un vecido aleatorio.
    ///
    /// Regresa el indice de la ciudad en el rango de las ciudades.
    ///
    fn get_vecino(&mut self) -> i64 {
        
        return self.random.random_range(0..self.solucion_actual.len()) as i64;
    }

    /// Funcion encargada de ejecutar el algoritmo de aceptacion por umbrales
    ///
    /// Genera soluciones a partir de una temperatura, la temperatura ira disminuyendo durante cada ejecucion del algoritmo una vez que el calculo de un lote mejora el promedio de las soluciones.
    ///
    /// # Example
    /// ```
    /// tsp.aceptacion_por_umbrales();
    /// ```
    pub fn aceptacion_por_umbrales (&mut self) {

        let e: f64 = 0.0001;
        let phi: f64 = 0.95;
        let _ = self.temperatura_inicial();
        self.promedio = 0.0;
        self.generar_primer_solucion();
        self.peso_solucion_actual = self.calcular_solucion();
        while self.temperatura > e {
            let mut q = f64::MAX;
            while self.promedio <= q{
                q = self.promedio;
                self.calcular_lote();
            }
            
            self.temperatura = self.temperatura * phi;
            

        } 
    }

    /// Funcion encargada de ejecutar el algoritmo de barrido.
    ///
    /// Busca mejorar las soluciones realizando un intercambio de ciudades cada vez que al hacer el intercambio de ciudades pueda mejorar la solucion actual del algoritmo.
    ///
    /// # Example
    /// ```
    /// tsp.barrido();
    /// ```
    pub fn barrido(&mut self){
        self.peso_solucion_actual = self.calcular_solucion();
        let mut a:usize = 0;
        while a < self.solucion_actual.len(){
            let mut i = a+1;
            while i < self.solucion_actual.len() {
                let c = self.peso_solucion_actual;
                let d = self.intercambiar_ciudades(a,i);
            
                if d < c {
                    self.peso_solucion_actual = d;
                    self.barrido();
                    return;
                }
                let _ = self.intercambiar_ciudades(a,i);
                i+=1;
            }
            a+=1;
        }
        return;
    }

    /// Funcion encargada de calcular la temperatura inicial.
    ///
    /// La función se encarga de calcular la temperatura inicial que nos proveera de un porcentaje de aceptacion}
    /// de las soluciones en el rango que nosotros proporcionamos, dicho esto se realiza haciendo uso de busqueda binaria.
    ///
    fn temperatura_inicial (&mut self) {
        let porc = 0.60;
        let t1:f64;
        let t2:f64;
        let mut t:f64 = self.temperatura;
        let mut p = self.porcentajes_aceptados(t);
        if (porc - p).abs() <= 0.0001 {
            return;
        }
        if p < porc {
            while p < porc {
                t = t*2.0;
                p = self.porcentajes_aceptados(t);
            }
            t1 = t/2.0;
            t2 = t;
        } else {
            while p > porc {
                t = t / 2.0;
                p = self.porcentajes_aceptados(t);
            }
            t1 = t;
            t2 = t*2.0;
        }

        self.temperatura = self.busqueda_binaria(t1,t2,porc);
    }

    /// Funcion que obtiene el porcentaje de aceptados
    ///
    /// La función se encarga de calcular el porcentaje de soluciones aceptadas para un tamaño de lote especifico
    /// 
    fn porcentajes_aceptados(&mut self, t:f64) -> f64{
        let mut c = 0;
        let mut i = 1;
        let l = 3500;
        let s = self.solucion_actual.clone();
        while i < l {
            let a = self.get_vecino();
            let b = self.get_vecino();
            let new_sol = self.intercambiar_ciudades(a as usize, b as usize);
            if new_sol <= self.peso_solucion_actual + t {
                c+=1;
                self.peso_solucion_actual = new_sol;
            } else {
                self.intercambiar_ciudades(a as usize, b as usize);
            }
            i+=1;
        }
        self.solucion_actual = s;
        return (c as f64)/(l as f64);
    }

    /// Función busqueda binaria temperatura.
    ///
    /// Funcion encargada de buscar la temperatura ideal para el porcentaje de soluciones aceptadas.
    ///
    /// Regresa la temperatura encontrada.
    ///
    fn busqueda_binaria(&mut self, t1:f64, t2:f64, porc:f64) -> f64{
        let tm = (t1 + t2)/2.0;
        if t2 - t1 < 0.0001 {
            return tm;
        }
        let p = self.porcentajes_aceptados(tm);
        if (porc - p).abs() < 0.0001 {
            return tm;
        }
        if p > porc {
            return self.busqueda_binaria(t1,tm,porc);
        }else {
            return self.busqueda_binaria(tm,t2,porc);
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::fs;
    use crate::db::CityDB;
    use crate::grafica::Grafica;
    fn generar_tsp(ruta: String) -> Tsp {

        let contenido = fs::read_to_string(ruta);

        let numeros: Vec<i64> = contenido.expect("No es un entero").trim().split(',').map(|s| s.parse::<i64>().expect("Error al convertir el numero")).collect();

        let mut cities = CityDB::new(&numeros);

        let _ = cities.cargar_datos();

        let g = Grafica::new(cities);

        println!("Arreglo: {:?}", numeros);

        let tsp = Tsp::new(1000.0, g, numeros, 75);

        return tsp;
        
    }
    
    #[test]
    fn ok_calcular_solucion() {        
        let mut tsp:Tsp = generar_tsp("inputs/input-40.tsp".to_string());
        assert_eq!(tsp.calcular_solucion(), 7598476.968976471);

        //let mut tsp = generar_tsp("inputs/input-150.tsp".to_string());
        //assert_eq!(tsp.calcular_solucion(), 6161590.480045998);
        
    }

    #[test]
    fn ok_intercambiar_ciudades() {
        let mut tsp:Tsp = generar_tsp("inputs/input-40.tsp".to_string());
        let res1 = tsp.calcular_solucion();
        tsp.peso_solucion_actual = res1;
        let res = tsp.intercambiar_ciudades(5,20);
        assert_eq!(tsp.calcular_solucion(), res);
    }
}

