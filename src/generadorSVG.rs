use std::cmp::Ordering;
use std::io::Write;
use std::fs::File;
use std::fs;

pub fn generar(datos: Vec<f64>, nombre: String) {
    let ruta = format!("svgs/{}.svg", nombre);
    let ancho = 600.0;
    let alto = 400.0;
    let max = datos.iter().cloned().fold(f64::MIN, f64::max);

    let mut svg_text = format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{0}" height="{1}">"#, ancho, alto);

    for i in 0..datos.len()-1 {
        let x1 = i as f64 / (datos.len()-1) as f64 * ancho;
        let y1 = alto - datos[i] / max * alto;
        let x2 = (i+1) as f64 / (datos.len()-1) as f64 * ancho;
        let y2 = alto - datos[i+1] / max * alto;

        svg_text += &format!(r#"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="blue" stroke-width="2" />"#, x1, y1, x2, y2);
    }
    svg_text += "</svg>";

    let mut file = File::create(ruta).unwrap();
    file.write_all(svg_text.as_bytes()).unwrap();
}

