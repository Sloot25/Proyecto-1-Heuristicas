mod db;
mod grafica;


use db::CityDB;
use grafica::Grafica;

fn main(){
    let mut cities = CityDB::new();

    let _ = cities.cargar_datos();

    let mut g = Grafica::new(cities);
    println!("Impresion de distancia Natural {}", g.distanciaNatural(1,7));
    println!("Impresion de peso {}", g.peso(1,7));
}
