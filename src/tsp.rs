struct Solucion {
    solucion: Vec<f64>,
    temperatura: f64,
    promedio: f64,
    soluciones_aceptadas: Vec<f64>
}

impl Solucion {
    fn new(temp: f64, sol: Vec<f64>) -> Self {
        Solucion {
            solucion: sol,
            temperatura: temp,
            promedio: 0.0,
        }
    }

    fn calcular_lote() {
        let mut c: i64 = 0;
        let mut i: i64 = 0;
        promedio = 0.0;
        let l: i64 = 100;
        let l2: i64 = 1 000 000;
        while c < l || i < l2 {
            
        }
    }

    fn intercambiar_ciudades(a: i64, b: i64) {
        let temp = a;
        solucion[a] = b;
        solucion[b] = a;
    }

    fn getVecino() -> i64{
        return 
    }
}

pub struct Tsp {
    db: CityDB,
    soluciones_aceptadas: Vec<f64>,
    solucion_actual: Solucion,
    
}

impl Tsp {
    pub fn new() -> Self {
        
    }
}
