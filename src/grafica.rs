
use crate::db::CityDB;
use std::boxed::Box;

pub struct Grafica {
    grafica: Box<[[f64; 1092]; 1092]>,
    db: CityDB,
}

impl Grafica {
    pub fn new(m: Box<[[f64; 1092]; 1092]>, base: CityDB) -> Self {
        Grafica { grafica: m, db:base, }
    }

    pub fn distanciaNatural(&mut self, u: i64, v: i64) -> f64{
        let r = 6373000.0;
        let c = 2.0 * self.getA(u, v).sqrt().atan2((1.0 - self.getA(u, v)).sqrt());
        return r * c;
    }

    fn getA (&mut self, u: i64, v: i64) -> f64 {
        let u_tupla = self.db.get_latitude_longitude(u);
        let v_tupla = self.db.get_latitude_longitude(v);

        let a = (((v_tupla.0 - u_tupla.0)/2.0).sin()).powf(2.0);

        let b = (((v_tupla.1 - u_tupla.0)/2.0).sin()).powf(2.0);

        return a + (u_tupla.0.cos() * v_tupla.0.cos() * b);
    }

    pub fn peso(&mut self, u: i64, v: i64) -> f64 {
        if self.grafica[u as usize][v as usize] != -1.0 {
            self.grafica[u as usize][v as usize] = self.distanciaNatural(u, v) * self.db.distanciaMaxima;
        }
        return self.grafica[u as usize][v as usize];
    }

    pub fn getVecino(&mut self, u: i64) -> i64 {
        return 2; 
    }
}
