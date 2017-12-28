extern crate rand;

use structs::conexion::Conexion;
use std::sync::{Arc};
use self::rand::{XorShiftRng, SeedableRng, Rng};
use self::rand::distributions::{IndependentSample, Range};

pub struct Ant{
    pub ciudad : usize,
    rng: XorShiftRng,
}



impl Ant{
    pub fn new(mut rng: XorShiftRng,num_ciudades: usize) -> Self{
        let between = Range::new(0,num_ciudades);
        Ant {
            ciudad: between.ind_sample(&mut rng),
            rng: rng,
        }
    }

    pub fn muevete(&mut self,matriz: &Arc<Vec<Vec<Conexion>>>, ciudades_visitar:&Vec<usize>) -> (usize,f64) {
        let mut probs = 0.0;
        for city in ciudades_visitar {
            probs += *matriz[self.ciudad][*city].probabilidad.lock().unwrap();
        }
        //tienes que agregar el rng
        //matriz[self.ciudad]
        (15,0.0)
    }
}
