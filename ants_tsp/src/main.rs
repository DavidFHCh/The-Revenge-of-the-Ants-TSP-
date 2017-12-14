//fn main() {
//    println!("Hello, world!");
//}

extern crate threadpool;
extern crate num_cpus;
extern crate ants_tsp;
extern crate rand;

use ants_tsp as ants;
use std::sync::{Arc};
use ants::conexion_bd::get_ciudades;
use rand::{XorShiftRng, SeedableRng, Rng};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use ants_tsp::structs::conexion::Conexion;
use ants_tsp::structs::queue_parll::QueueParll;
use ants_tsp::structs::ant::Ant;
use rand::distributions::{IndependentSample, Range};



static RECORRIDOS: usize = 2000000;

fn computation(matriz: Arc<Vec<Vec<Conexion>>>,ant: Ant, evaporar: bool) -> String{
        //let mut shared = matriz;
        let conexiones = rand::thread_rng().choose(&matriz);
        let conexion = rand::thread_rng().choose(&conexiones.unwrap());
        let mut prob = conexion.unwrap().probabilidad.lock().unwrap();

        let formatted1 = format!("{} ", prob);
        *prob += 1.0;
        let formatted2 = format!("{} ", prob);

        let formatted3 = format!("{} {}", conexion.unwrap().ciudad1,conexion.unwrap().ciudad2);

        let res = format!("{} {} {}",formatted1,formatted2, formatted3);
        res
}

fn main() {


let ciudades_matriz = get_ciudades().unwrap();

let matriz = ciudades_matriz.1;

let mut ants: QueueParll = QueueParll::new();

let semilla = 666;
let seed = [semilla, semilla*3, semilla*5, semilla*7];
let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
let conjunto_ciudades: Vec<usize> = vec![1,2,3,4,5,6,7,8,9,10];

for _x in 0..RECORRIDOS {
    let between = Range::new(0,conjunto_ciudades.len());
    let ant: Ant = Ant::new(between.ind_sample(&mut rng));
    ants.push(ant);
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
    pool.execute(move|| {
        tx.send(computation(matriz,ant,evap)).unwrap();
    });
    println!("{:?}",rx.recv().unwrap());
}

println!("{:?}",rx.recv().unwrap());
}
