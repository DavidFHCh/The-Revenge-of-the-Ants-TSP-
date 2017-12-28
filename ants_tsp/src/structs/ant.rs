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
    pub fn new(seed: [u32;4],num_ciudades: usize) -> Self{
        let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
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

        //aqui tienes la suma de probs, se da un num aleatorio en el rango 0..probs y se ve a cual corresponde, de acuerdo al orden de ciudades_visitar.

        let between = Range::new(0.0,probs);
        let mut val_obt = between.ind_sample(&mut self.rng);
        let mut dis = 0.0;
        for city in ciudades_visitar {
            if val_obt < *matriz[self.ciudad][*city].probabilidad.lock().unwrap(){
                self.ciudad = city.clone();
                dis = matriz[self.ciudad][*city].distancia.clone();
            } else {
                val_obt -= *matriz[self.ciudad][*city].probabilidad.lock().unwrap();
            }

        }
        //matriz[self.ciudad]
        (self.ciudad,dis)
    }
}
