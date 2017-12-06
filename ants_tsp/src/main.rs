//fn main() {
//    println!("Hello, world!");
//}

extern crate threadpool;
extern crate num_cpus;
extern crate ants_tsp;
extern crate rand;

use ants_tsp as ants;
use std::ops::Deref;
use std::sync::{Mutex,Arc};
use ants::conexion_bd::get_ciudades;
use rand::{thread_rng, Rng};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use ants_tsp::structs::conexion::Conexion;

fn computation(matriz: Arc<Mutex<Vec<Vec<Conexion>>>>) -> String{
        let mut shared = matriz.lock().unwrap();
        let mut rng = thread_rng();
        let adys = rng.choose(shared.deref()).unwrap();
        let mut conexion = rng.choose(&adys).unwrap();
        let mut prob = conexion.probabilidad;

        let formatted1 = format!("{} ", conexion.probabilidad);
        let mut prob = conexion.probabilidad.clone();
        let formatted2 = format!("{} ", conexion.probabilidad);

        let res = format!("{} {}",formatted1,formatted2);
        res
}

fn main() {
let n_workers = num_cpus::get();
let n_jobs = 8;
let pool = ThreadPool::new(n_workers);

let ciudades_matriz = get_ciudades().unwrap();
//let ciudades = ciudades_matriz.0;
let matriz = Arc::new(Mutex::new(ciudades_matriz.1));


let (tx, rx) = channel();
for _ in 0..n_jobs {
    let tx = tx.clone();
    let matriz = matriz.clone();
    pool.execute(move|| {
        tx.send(computation(matriz)).unwrap();

    });
    println!("{:?}",rx.recv().unwrap());
    println!("WHAAAAAAAAAAAAAAAAT");
}

println!("{:?}",rx.recv().unwrap());
}
