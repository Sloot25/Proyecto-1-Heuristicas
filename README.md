# Proyecto-1-Heuristicas

El presente trabajo es una implementación de recocido simulado para aproximar soluciones a una instancia del problema del TSP. La implementación corresponde al curso de ``Heuristicas de Optimización Combinatoria'' y fue realizado en Rust. Donde se puede ver la realización del ejecicio y el desarrollo hasta su optimización a la obtención del mejor resultado posible.

## Instancia del problema 

La instancia del problema se encuentra dentro de la carpeta inputs, donde cada archivo corresponde al conjunto de ciudades a las cuales queremos encontrar la mejor trayectoria de menor coste que podamos ser capaces de encontrar. 

## Base de datos

El archivo tsp.db contiene la base de datos correspondiente a todas las ciudades que conforman nuestra base de datos del proyecto, siendo esta la base de datos que vamos a cargar.

## Dependencias

Como en todo proyecto, las dependencias utilizadas pueden encontrarse en Cargo.toml, pero son las siguientes:

- rusqlite
- rand
- chrono
- rdp
- svg
- geo 
- plotters

## Documento del proyecto

El Documento del proyecto se encuentra dentro de la carpeta Documento, el cual es un LaTex y se encuentra compilado en primera instancia a un pdf. 

## Ejecucion

El proyecto se ejecuta con 

``` 
    cargo run -r -- 
```

Donde los parametros dados dependen de la opción solicitada 

- Generador de grafica SVG (Sin los puntos de mejoria):
``` 
    cargo run -r -- -s <nombre archivo con el arreglo a graficar> <nombre archivo a imprimir>
```

- Realizar barrido a una solucion:

``` 
    cargo run -r -- -b <nombre archivo con el arreglo con solucion> <nombre archivo a imprimir>
```

- Evaluar una solución:

``` 
    cargo run -r -- -e <nombre archivo con el arreglo con solucion>
```

- Ejecutar el algoritmo para una semilla

``` 
    cargo run -r -- -o <archivo de ciudades> <semilla> <nombre archivo salida>
```

- Ejecutar el algoritmo para una semilla con svg

``` 
    cargo run -r -- -o <archivo de ciudades> <semilla> <nombre archivo salida> -s
```

- Ejecutar el algoritmo para inteervalo de semillas

``` 
    cargo run -r -- -i <archivo de ciudades> <semilla inicial> <semilla final> <nombre archivo salida>
```

- Ejecutar el algoritmo para una semilla con svg

``` 
    cargo run -r -- -i <archivo de ciudades> <semilla inicial> <semilla final> <nombre archivo salida> -s
```
