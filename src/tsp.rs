

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
    mejor_solucion: f64,
    semilla: i64,
    normalizador: f64,
}

impl Tsp {
    pub fn new(temperatura: f64, grafica: Grafica, solucion_actual: Vec<i64>, semilla: i64, normalizador: f64) -> Self {

        Tsp {
            grafica,
            solucion_actual,
            soluciones_aceptadas: Vec::new(),
            temperatura,
            promedio: 0.0,
            mejor_solucion: 0.0,
            semilla,
            normalizador,
        }

        
    }

    fn calcular_lote(&mut self) {
        let mut c: i64 = 0;
        let mut r: f64 = 0.0;
        let mut i: i64 = 0;
        let l = 1000;
        let l2 = 1000000000;
        
        while c < l || i < l2 {
            let a = self.get_vecino();
            let b = self.get_vecino();
            let ant_sol = self.calcular_solucion();
            let _ = self.intercambiar_ciudades(a, b);
            let new_sol = self.calcular_solucion();
            if self.calcular_solucion() <= (ant_sol + self.temperatura) {
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

        self.promedio = r/(self.soluciones_aceptadas.len() as f64);
    }

    pub fn calcular_solucion(&mut self) -> f64 {
        let i: i64 = 0;
        let j: i64 = 1;
        let mut res: f64 = 0.0;
        while j < (self.solucion_actual.len() as i64) {
            res = res + self.grafica.peso(i, j);
        }
        return res / self.normalizador;
    }

    pub fn generar_primer_solucion(&mut self) {
        let mut rng = StdRng::seed_from_u64(self.semilla as u64);
        let i = 0;
        while i < self.solucion_actual.len()-2 {
            let k: usize = rng.gen_range(0..self.solucion_actual.len());
            self.solucion_actual[i] = self.solucion_actual[k];
        }
    }

    fn intercambiar_ciudades(&mut self, a: i64, b: i64){
        let temp = a;
        self.solucion_actual[a as usize] = b;
        self.solucion_actual[b as usize] = temp;
    }

    fn get_vecino(&mut self) -> i64 {
        let mut rng = StdRng::seed_from_u64(self.semilla as u64);
        return self.solucion_actual[rng.gen_range(0..self.solucion_actual.len())];
    }

    pub fn aceptacion_por_umbrales (&mut self) {
        let e: f64 = 0.0001;
        let phi: f64 = 0.83;
        self.promedio = 0.0;
        while self.temperatura > e {
            let mut q = f64::MAX;
            while self.promedio <= q {
                q = self.promedio;
                self.calcular_lote();
            }
            self.temperatura = self.temperatura * phi;
        } 
    }

}
