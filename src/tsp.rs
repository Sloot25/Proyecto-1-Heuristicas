use rand::SeedableRng;
use rand::Rng;
use rand::rngs::StdRng;
use crate::grafica::Grafica;


pub struct Tsp {
    grafica: Grafica,
    pub soluciones_aceptadas: Vec<f64>,
    pub solucion_actual: Vec<i64>,
    temperatura: f64,
    promedio: f64,
    pub mejor_solucion: f64,
    semilla: i64,
    normalizador: f64,
    random: StdRng,
}

impl Tsp {
    pub fn new(temperatura: f64, grafica: Grafica, solucion_actual: Vec<i64>, semilla: i64) -> Self {
        let mut rng = StdRng::seed_from_u64(semilla as u64);
        let mut normalizador = Self::get_normalizador(&grafica.db.distancias_tsp, solucion_actual.len());
        Tsp {
            grafica,
            solucion_actual,
            soluciones_aceptadas: Vec::new(),
            temperatura,
            promedio: 0.0,
            mejor_solucion: f64::MAX,
            semilla,
            normalizador,
            random: rng,
        }

        
    }

    fn calcular_lote(&mut self) {
        let mut c: i64 = 0;
        let mut r: f64 = 0.0;
        let mut i: i64 = 0;
        let l = 50000;
        let l2 = 100000;
        
        while c < l || i < l2 {
            let a = self.get_vecino();
            let b = self.get_vecino();
            let ant_sol = self.calcular_solucion();
            
            let _ = self.intercambiar_ciudades(a, b);
            let new_sol = self.calcular_solucion();
                        
            if new_sol < (ant_sol + self.temperatura) {
                c = c+1;
                r = r + new_sol;
                self.soluciones_aceptadas.push(new_sol);

                println!("Solucion actual {:?}", self.solucion_actual);
                println!("Valor: {:?}", new_sol);
                

                if new_sol < self.mejor_solucion {
                    self.mejor_solucion = new_sol;

                }
            } else {
                self.intercambiar_ciudades(a,b);
            }
            i = i + 1;
        }

        self.promedio = r/(l as f64);
    }

    pub fn calcular_solucion(&mut self) -> f64 {
        let mut i: usize = 0;
        let mut j: usize = 1;
        let mut res: f64 = 0.0;
        
        while j < self.solucion_actual.len()  {
            res = res + self.grafica.peso(self.solucion_actual[i], self.solucion_actual[j]);
            i = i+1;
            j = j+1;
        }
       // res = res + self.grafica.peso(self.solucion_actual[self.solucion_actual.len() - 1], self.solucion_actual[0]);
        let s:f64 = res/self.normalizador;
        return s;
    }

    fn get_normalizador(lista_ordenada: &Vec<f64>, n: usize) -> f64 {
        let mut i = lista_ordenada.len() - n + 1;
        let mut normalizador: f64 = 0.0;
        while i < lista_ordenada.len(){
            normalizador = normalizador + lista_ordenada[i];
            i = i+1;
        }
        return normalizador;
    }

    pub fn generar_primer_solucion(&mut self) {
        let mut i: i64 = 0;
        while i < (self.solucion_actual.len() as i64) {
            let k: usize = self.random.gen_range(0..self.solucion_actual.len());
            self.intercambiar_ciudades(i,k as i64);
            i = i+1;
        }
    }

    fn intercambiar_ciudades(&mut self, a: i64, b: i64){
        let temp = self.solucion_actual[a as usize];
        self.solucion_actual[a as usize] = self.solucion_actual[b as usize];
        self.solucion_actual[b as usize] = temp;
    }

    fn get_vecino(&mut self) -> i64 {
        
        return self.random.gen_range(0..self.solucion_actual.len()) as i64;
    }

    pub fn aceptacion_por_umbrales (&mut self) {
        let e: f64 = 0.0001;
        let mut phi: f64 = 0.8;
        self.promedio = 0.0;
        while self.temperatura > e {
            let mut q = f64::MAX;
            while self.promedio <= q {

                q = self.promedio;
                self.calcular_lote();
            }
            
            if self.soluciones_aceptadas[self.soluciones_aceptadas.len()-1] < 1.0 || self.temperatura < 50000.0 {
                phi = 0.93;
            }

            self.temperatura = self.temperatura * phi;
            
            println!("Temperatura {}", self.temperatura);
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

        let mut g = Grafica::new(cities);

        println!("Arreglo: {:?}", numeros);

        let mut tsp = Tsp::new(1000.0, g, numeros, 75);

        return tsp;
        
    }
    
    #[test]
    fn ok_calcular_solucion() {        
        let mut tsp:Tsp = generar_tsp("inputs/input-40.tsp".to_string());
        assert_eq!(tsp.calcular_solucion(), 7598476.968976471);

        //let mut tsp = generar_tsp("inputs/input-150.tsp".to_string());
        //assert_eq!(tsp.calcular_solucion(), 6161590.480045998);
        
    }
}

