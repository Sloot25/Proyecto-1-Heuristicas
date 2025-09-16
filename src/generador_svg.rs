use geo::{LineString, Point};
use geo::algorithm::simplify::Simplify;
use svg::node::element::{Circle, Polyline, Line, Text, Rectangle};
use svg::Document;


pub fn generar(datos: Vec<f64>, datos_puntos: Vec<(f64, bool)> ,nombre: String) {
    let ruta = format!("svgs/{}.svg", nombre);
    if datos.is_empty(){
        println!("No hay datos para generar el SVG.");
        return;
    }

    let puntos: Vec<Point<f64>> = datos
        .iter()
        .enumerate()
        .map(|(i, valor)| Point::new(i as f64, *valor))
        .collect();

    let linea_original = LineString::from(puntos.clone());
    
    let puntos_importantes: Vec<[f64; 2]> = datos_puntos
        .iter()
        .enumerate()
        .filter(|(_, (_, es_mejora))| *es_mejora)
        .map(|(i, (valor, _))| [i as f64, *valor])
        .collect();

    let epsilon = 2.0;
    let linea_rdp = linea_original.simplify(epsilon);

    let width = 2800.0;
    let height = 1600.0;
    let margin = 120.0;

    let (min_y, max_y) = datos.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), valor| {
        (min.min(*valor), max.max(*valor))
    });

    let max_x = (datos.len() - 1) as f64;

    let map_svg = |x:f64, y:f64| -> (f64, f64) {
        let svg_x = (x / max_x) * (width - 2.0 * margin) + margin;
        let svg_y = (1.0 - (y- min_y) / (max_y - min_y)) * (height - 2.0 * margin) + margin;
        (svg_x, svg_y)
    };

    let point_string = linea_rdp
        .points()
        .map(|p| {
            let (svg_x, svg_y) = map_svg(p.x(),p.y());
            format!("{:.2},{:.2}", svg_x, svg_y)
        })
        .collect::<Vec<String>>()
        .join(" ");

    let linea = Polyline::new()
        .set("fill", "none")
        .set("stroke", "blue")
        .set("stoke-width", 2)
        .set("points", point_string);

    let puntos_mejora = puntos_importantes
        .iter().map(|&[x,y]| {
            let (svg_x, svg_y) = map_svg(x,y);
            Circle::new()
                .set("cx", ((svg_x * 100.0).round())/100.0)
                .set("cy", ((svg_y * 100.0).round())/100.0)
                .set("r", 3)
                .set("fill", "red")
        });
    
    let fondo = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", width)
        .set("height", height)
        .set("fill", "white");

    let eje_x = Line::new()
        .set("x1", margin)
        .set("y1", height - margin)
        .set("x2", width - margin)
        .set("y2", height - margin)
        .set("stroke", "black")
        .set("stroke-width", 2);

    let eje_y = Line::new()
        .set("x1", margin)
        .set("y1", margin)
        .set("x2", margin)
        .set("y2", height - margin)
        .set("stroke", "black")
        .set("stroke-width", 2);

    let etiqueta_x = Text::new("Ejecución")
        .set("x", width / 2.0)
        .set("y", height - 10.0)
        .set("text-anchor", "middle")
        .set("font-size", 20)
        .set("fill", "black")
        .add(svg::node::Text::new(""));

    let etiqueta_y = Text::new("Peso Solucion")
        .set("x", 20.0)
        .set("y", height / 2.0)
        .set("transform", format!("rotate(-90, {}, {})", 20.0, height / 2.0))
        .set("text-anchor", "middle")
        .set("font-size", 20)
        .set("fill", "black")
        .add(svg::node::Text::new(""));
    
    let mut file = Document::new().set("viewBox", (0,0,width, height))
        .add(fondo)
        .add(eje_x)
        .add(eje_y)
        .add(etiqueta_x)
        .add(etiqueta_y)
        .add(linea);
    
    for p in puntos_mejora {
        file = file.add(p);
    }

    
    
    svg::save(ruta, &file).expect("No se pudo guardar el SVG");
    
}

