mod db;
mod grafica;
mod tsp;
mod generadorSVG;

use db::CityDB;
use grafica::Grafica;
use std::fs::File;
use std::fs;
use std::env;
use tsp::Tsp;

use std::thread;
use std::io::Write;
use chrono::Local;
use std::path::PathBuf;
use crate::generadorSVG::generar;

fn lanzar_tsp(semilla: i64, g: Grafica, numeros: Vec<i64>, cadena: String) -> std::io::Result<()> {
    let mut tsp = Tsp::new(50000.0, g, numeros, semilla);
    let _ = tsp.generar_primer_solucion();
    let _ = tsp.aceptacion_por_umbrales();

    let ruta = format!("resultados/{}_semilla_{}_{}.txt", cadena,semilla, Local::now().format("%Y-%m-%d_%H-%M-%S") );
    let mut file = File::create(ruta)?;

    let contenido = format!("Soluciones Aceptadas: {:?}\n Solucion Actual {:?}\n Valor: {}\n ArregloMs: {:?} \n Mejor solucion {}\n Semilla: {}", tsp.soluciones_aceptadas, tsp.solucion_actual, tsp.peso_solucion_actual,tsp.mejor_solucion_arr , tsp.mejor_solucion, semilla);

    println!("Valor {} Semilla {}", tsp.mejor_solucion, semilla);
    
    file.write_all(contenido.as_bytes())?;
    
    Ok(())
    
}

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "-s" {
        let contenido = fs::read_to_string(&args[2]);
        
        let numeros: Vec<f64> = contenido.expect("No es un entero").trim().split(',').map(|s| s.trim().parse::<f64>().expect("Error al convertir el numero")).collect();

        generar(numeros, args[3].clone());
        return;

    } else if args.len() > 1 && args[1] == "-e" {
        let contenido = fs::read_to_string(&args[2]);
        let numeros: Vec<i64> = contenido.expect("No es un entero").trim().split(',').map(|s| s.trim().parse::<i64>().expect("Error al convertir el numero")).collect();
        let mut cities = CityDB::new(&numeros);
        let _ = cities.cargar_datos();
        let mut g = Grafica::new(cities);
        let mut tsp = Tsp::new(50000.0, g, numeros, 0);
        println!("Resultado: {}", tsp.calcular_solucion());
        return;
    }else if args.len() < 5 {
        println!("Los argumentos son: <tipo de consulta> <file> <1er semilla> <2da semilla?> <nombre inicial archivo salida>");
        return;
    }

    let contenido = fs::read_to_string(&args[2]);

    let numeros: Vec<i64> = contenido.expect("No es un entero").trim().split(',').map(|s| s.parse::<i64>().expect("Error al convertir el numero")).collect();

    let mut cities = CityDB::new(&numeros);

    let _ = cities.cargar_datos();

    let mut g = Grafica::new(cities);


    if args[1] == "-o" {
        let semilla = args[3].parse::<i64>().expect("Error al parsear semilla");
        lanzar_tsp(semilla, g, numeros, args[4].clone()).unwrap();
    } else if args[1] == "-i" {
        let semilla1 = args[3].parse::<i64>().expect("Error al parsear semilla");
        let semilla2 = args[4].parse::<i64>().expect("Error al parsear semilla");
        let semillas: Vec<i64> = (semilla1..semilla2).collect();
        let mut handles = Vec::new();

        for semilla in semillas {
            let g_clon = g.clone();
            let numeros_clon = numeros.clone();
            let cadena = args[5].clone();
            let handle = thread::spawn( move || {
                lanzar_tsp(semilla, g_clon, numeros_clon, cadena).unwrap();
            });
            
            handles.push(handle);
            
            if handles.len() >= 6 {
                let handle = handles.remove(0);
                handle.join().unwrap();
            }
        }

    } 


}
