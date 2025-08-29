mod db;
mod grafica;
mod tsp;

use db::CityDB;
use grafica::Grafica;
use std::fs;
use std::env;
use tsp::Tsp;

fn main(){
    let args: Vec<String> = env::args().collect();
    
    let contenido = fs::read_to_string(&args[1]);

    let numeros: Vec<i64> = contenido.expect("No es un entero").trim().split(',').map(|s| s.parse::<i64>().expect("Error al convertir el numero")).collect();

    let mut cities = CityDB::new(&numeros);

    let _ = cities.cargar_datos();

    let mut g = Grafica::new(cities);

    let semilla = args[2].parse::<i64>().expect("Error al parsear semilla");
    
    let mut tsp = Tsp::new(50000.0, g, numeros, semilla);
    let _ = tsp.generar_primer_solucion();
    let _ = tsp.aceptacion_por_umbrales();
    println!("Solucion Actual {:?}", tsp.solucion_actual);
    println!("Valor {:?}", tsp.calcular_solucion());
    println!("Soluciones {:?}", tsp.soluciones_aceptadas);
    println!("Mejor solucion {}", tsp.mejor_solucion);
    println!("Semilla {}", semilla);

}
