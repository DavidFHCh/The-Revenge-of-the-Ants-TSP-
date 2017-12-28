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
use std::sync::{Arc};
use ants::conexion_bd::get_ciudades;
use rand::{XorShiftRng, SeedableRng, Rng};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use ants_tsp::structs::conexion::Conexion;
use ants_tsp::structs::queue_parll::QueueParll;
use ants_tsp::structs::ant::Ant;
use ants_tsp::structs::solucion::Solucion;
use rand::distributions::{IndependentSample, Range};
use config::{Config, File, FileFormat, Value};



static RECORRIDOS: usize = 2000;
static AUMENTO_FEROMONA: f64 = 0.2;
static DISMINUCION_FEROMONA: f64 = 0.2;

fn recorrido_hormiga(matriz: Arc<Vec<Vec<Conexion>>>,mut ciudades_visitar:Vec<usize>,mut ant: Ant, evaporar: bool) -> String{
        //let mut shared = matriz;
        let conexiones = rand::thread_rng().choose(&matriz);
        let mut con = rand::thread_rng().choose(&conexiones.unwrap());
        let mut conexion = con.unwrap();
        let mut solucion = Solucion::new();
        let num_c = ciudades_visitar.len();
        for i in 0..num_c {
            let movimiento = ant.muevete(&matriz,&ciudades_visitar);
            solucion.f_obj += movimiento.1;
            solucion.solucion.push(movimiento.0.clone());
            //ciudades_visitar.remove_item(&movimiento.0);
            //se eliminara el que se visito afuera de la funcion
        }

        if ciudades_visitar.len() == 0 {
            println!("true");
        } else {
            println!("{}", ciudades_visitar.len());
        }


        if conexion.ciudad1 != 0 && conexion.ciudad2 != 0{
            *conexion.feromona.lock().unwrap() += AUMENTO_FEROMONA;

            let mut prob = conexion.probabilidad.lock().unwrap();
            let formatted1 = format!("{} ", prob);
            *prob = (*conexion.probabilidad_base.lock().unwrap()) * (*conexion.feromona.lock().unwrap());
            //let formatted2 = format!("{} ", prob);

            let formatted3 = format!("{} {}", conexion.ciudad1,conexion.ciudad2);

            let res = format!("{} {} {}",formatted1,formatted3,ant.ciudad);
            return res
        } else {
            let res = format!("nada {}", ant.ciudad);
            return res
        }
}

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

fn main() {
    let mut c = Config::new();
    let ciudades_matriz = get_ciudades().unwrap();
    let matriz = ciudades_matriz.1;
    c.merge(File::new("Ajustes", FileFormat::Toml).required(true)).expect("NO HAY ARCHIVO DE CONFIGURACION 'Ajustes.toml'");

    let semillas: Vec<u32> = to_u32_vec(c.get_array("seeds").expect("No hay lista de semillas declarada en Ajustes.toml"));
    let conjunto_ciudades = to_usize_vec(c.get_array("ciudad_ids").expect("No hay lista de ids de ciudades declarada en Ajustes.toml"));
    //println!("{}", conjunto_ciudades.len());
    let mut ants: QueueParll = QueueParll::new();

    for semilla in semillas {
        for _x in 0..RECORRIDOS {
            let seed = [semilla, semilla*3, semilla*5, semilla*7];
            let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
            //let between = Range::new(0,conjunto_ciudades.len());
            let ant: Ant = Ant::new(rng,conjunto_ciudades.len());
            ants.push(ant);
        }
    }

    let n_workers = num_cpus::get();
    let n_jobs = RECORRIDOS;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();

    for _ in 0..n_jobs {
        let ant_evap = ants.pop();
        let ant = ant_evap.0;
        let evap = ant_evap.1;
        let tx = tx.clone();
        let matriz = matriz.clone();
        let a_visitar = conjunto_ciudades.clone();

        pool.execute(move|| {
            tx.send(recorrido_hormiga(matriz,a_visitar,ant,evap)).unwrap();
        });
        rx.recv();
        //println!("{:?}",rx.recv().unwrap());
    }



    //println!("{:?}",rx.recv().unwrap());
}
