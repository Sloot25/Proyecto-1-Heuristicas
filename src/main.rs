mod db;

use db::CityDB;

fn main(){
    let mut cities = CityDB::new();
    let _ = cities.cargar_datos();
    for(id, city) in cities.data {
        println!("ID {} => {:?}", id, city);
    }

}
