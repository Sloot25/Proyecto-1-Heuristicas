mod db;
mod grafica;


use db::CityDB;
use grafica::Grafica;

fn main(){
    println!("Holaaaaaaaaaaaaa");
    
    let mut cities = CityDB::new();

    println!("Hola");

    //let _ = cities.cargar_datos();

    //let data = std::mem::replace(&mut cities.data, Box::new([[-1.0; 1092]; 1092]));
    //let mut g = Grafica::new(data, cities);
    //println!("Impresion de distancia Natural {}", g.distanciaNatural(1,7));
    //println!("Impresion de peso {}", g.peso(1,7));
}
