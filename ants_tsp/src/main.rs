//fn main() {
//    println!("Hello, world!");
//}
#![feature(vec_remove_item)]
extern crate threadpool;
extern crate num_cpus;
extern crate ants_tsp;
extern crate rand;
extern crate config;

use ants_tsp as ants;
//use std::sync::{Arc};
use ants::conexion_bd::get_ciudades;
use rand::{XorShiftRng, SeedableRng, Rng};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use ants_tsp::structs::conexion::Conexion;
use ants_tsp::structs::ant::Ant;
use ants_tsp::structs::city::City;
use ants_tsp::structs::solucion::Solucion;
use rand::distributions::{IndependentSample, Range};
use config::{Config, File, FileFormat, Value};



static RECORRIDOS: usize = 2000;
static NUM_HORMIGAS: usize = 20;
static AUMENTO_FEROMONA: f64 = 0.0;
static DISMINUCION_FEROMONA: f64 = 20.0;


fn to_usize_vec(values: Vec<Value>) -> Vec<usize> {
    let mut v = Vec::with_capacity(values.len());
    for vs in values.clone() {
        v.push(vs.into_int().expect("Error convirtiendo a i64") as usize);
    }
    v
}

fn to_u32_vec(values: Vec<Value>) -> Vec<u32> {
    let mut v = Vec::with_capacity(values.len());
    for vs in values.clone() {
        v.push(vs.into_int().expect("Error convirtiendo a i64") as u32);
    }
    v
}

fn set_all_false(conj_ciudades:&mut Vec<City>) {
    for city in conj_ciudades {
        city.set_false_visited();
    }
}

fn main() {
    //let mut soluciones = Vec::new();
    let mut c = Config::new();
    let ciudades_matriz = get_ciudades().unwrap();
    let matriz = ciudades_matriz.1;
    c.merge(File::new("Ajustes", FileFormat::Toml).required(true)).expect("NO HAY ARCHIVO DE CONFIGURACION 'Ajustes.toml'");

    let semillas: Vec<u32> = to_u32_vec(c.get_array("seeds").expect("No hay lista de semillas declarada en Ajustes.toml"));
    let mut conjunto_ciudades = to_usize_vec(c.get_array("ciudad_ids").expect("No hay lista de ids de ciudades declarada en Ajustes.toml"));
    //println!("{}", conjunto_ciudades.len());
    //let conjunto_ciudades_1 = conjunto_ciudades.unwrap();
    let mut ciudades_a_visitar = Vec::new();
    for ciudad in &conjunto_ciudades {
        ciudades_a_visitar.push(City::new(*ciudad));
    }

    println!("{:?}", conjunto_ciudades);
    let mut hormigas = Vec::new();
    for num in 0..NUM_HORMIGAS {
        hormigas.push(Ant::new(0));
    }

    for semilla in semillas {
        let seed = [semilla, semilla*3, semilla*5, semilla*7];
        let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
        for _i in 0..RECORRIDOS {
            for hormiga in &mut hormigas {
                set_all_false(&mut ciudades_a_visitar);
                hormiga.set_ciudad(rng.choose(&ciudades_a_visitar).unwrap().ciudad);

            }
        }
    }


}
