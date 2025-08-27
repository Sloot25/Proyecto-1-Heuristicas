

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
    random: StdRng,
}

impl Tsp {
    pub fn new(temperatura: f64, grafica: Grafica, solucion_actual: Vec<i64>, semilla: i64, normalizador: f64) -> Self {
        let mut rng = StdRng::seed_from_u64(semilla as u64);
        Tsp {
            grafica,
            solucion_actual,
            soluciones_aceptadas: Vec::new(),
            temperatura,
            promedio: 0.0,
            mejor_solucion: 0.0,
            semilla,
            normalizador,
            random: rng,
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
                        
            if self.calcular_solucion() < (ant_sol + self.temperatura) {
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
        let mut i: usize = 0;
        let mut j: usize = 1;
        let mut res: f64 = 0.0;
        
        while j < self.solucion_actual.len()  {
            res = res + self.grafica.peso(self.solucion_actual[i], self.solucion_actual[j]);
            i = i+1;
            j = j+1;
        }
        return res / self.normalizador;
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
