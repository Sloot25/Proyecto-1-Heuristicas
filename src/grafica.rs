use std::f64::consts::PI;
use crate::db::CityDB;


#[derive(Clone)]
pub struct Grafica {
    pub db: CityDB,
}

impl Grafica {
    pub fn new(base: CityDB) -> Self {
        Grafica {db:base, }
    }

    pub fn distancia_natural(&mut self, u: i64, v: i64) -> f64{
        let r = 6373000.0;
        let c = 2.0 * self.get_a(u, v).sqrt().atan2((1.0 - self.get_a(u, v)).sqrt());
        return r * c;
    }

    fn get_a (&mut self, u: i64, v: i64) -> f64 {
        let u_tupla = self.db.get_latitude_longitude(u);
        let v_tupla = self.db.get_latitude_longitude(v);

        let rad = PI/180.0;
        
        let u_latitude_radianes = u_tupla.0 * rad;
        let u_longitude_radianes = u_tupla.1 * rad;

        let v_latitude_radianes = v_tupla.0 * rad;
        let v_longitude_radianes = v_tupla.1 * rad;
        
        
        let a = (((v_latitude_radianes - u_latitude_radianes)/2.0).sin()).powf(2.0);

        let b = (((v_longitude_radianes - u_longitude_radianes)/2.0).sin()).powf(2.0);

        return a + (u_latitude_radianes.cos() * v_latitude_radianes.cos() * b);
    }

    pub fn peso(&mut self, u: i64, v: i64) -> f64 {
        if self.db.data[(u*1093 +  v) as usize] == -1.0 {
            self.db.data[(u*1093 + v) as usize] = self.distancia_natural(u, v) * self.db.distancias_tsp[self.db.distancias_tsp.len()-1];
        }

        return self.db.data[(u*1093 + v) as usize];
    }

}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::fs;
    
    fn generar_numeros() -> Vec<i64>{

        let contenido = fs::read_to_string("inputs/input-40.tsp".to_string());

        let numeros: Vec<i64> = contenido.expect("No es un entero").trim().split(',').map(|s| s.parse::<i64>().expect("Error al convertir el numero")).collect();
        return numeros;
    }
    
    #[test]
    fn ok_distancia_natural(){
        let mut cities = CityDB::new(&generar_numeros());
        let _ = cities.cargar_datos();
        let mut g = Grafica::new(cities);

        let a1: i64 = 2999396;
        let a2: i64 = 1158707;
        
        assert_eq!(a1, g.distancia_natural(1,7) as i64);
        assert_eq!(a1, g.distancia_natural(7,1) as i64);
        assert_eq!(a2, g.distancia_natural(1,9) as i64);

    }

    #[test]
    fn ok_peso() {
        let mut cities = CityDB::new(&generar_numeros());
        let _ = cities.cargar_datos();
        let mut g = Grafica::new(cities);

        let a1: f64 = 2999396.229999999982;
        let a2: f64 = 1158707.310000000055;

        assert_eq!(a1, g.peso(1,7));
        assert_eq!(a1, g.peso(7,1));
        assert_eq!(a2, g.peso(1,9));
        assert_eq!(a2, g.peso(9,1));

    }
} 
