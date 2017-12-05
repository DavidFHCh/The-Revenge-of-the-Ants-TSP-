//fn main() {
//    println!("Hello, world!");
//}

extern crate threadpool;
extern crate num_cpus;
extern crate ants_tsp;
extern crate rand;

use ants_tsp as ants;
use std::sync::{Mutex,Arc};
use ants::conexion_bd::get_ciudades;
use rand::{thread_rng, Rng};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
fn main() {
let n_workers = num_cpus::get();
let n_jobs = 8;
let pool = ThreadPool::new(n_workers);

let ciudades_matriz = get_ciudades().unwrap();
//let ciudades = ciudades_matriz.0;
let matriz = Arc::new(Mutex::new(ciudades_matriz.1));


let (tx, _rx) = channel();
for _ in 0..n_jobs {
    let tx = tx.clone();
    let matrix = matriz.clone();
    pool.execute(move|| {
        tx.send(1).expect("channel will be there waiting for the pool");
        while true {
            let mut shared = matrix.lock().unwrap();
            let mut rng = thread_rng();
            let adys = rng.choose(shared).unwrap();
            let mut conexion = rng.choose(&adys).unwrap();
            println!("{}",conexion.probabilidad);
            let mut prob = conexion.probabilidad.clone();

            println!("{}",conexion.probabilidad);
        }
    });
}


}
