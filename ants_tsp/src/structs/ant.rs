extern crate rand;

use structs::conexion::Conexion;
use structs::city::City;
use std::sync::{Arc};
use self::rand::{XorShiftRng, SeedableRng, Rng};
use self::rand::distributions::{IndependentSample, Range};

#[derive(Clone)]
pub struct Ant{
    pub ciudad : usize,
    pub visitados: Vec<City>,
}

impl Ant {
    pub fn new(city: usize) -> Self {
        Ant {
            ciudad: city,
            visitados: Vec::new(),
        }
    }

    pub fn clean(&mut self) {
        self.ciudad = 0;
    }

    pub fn set_ciudad(&mut self, ciudad: usize) {
        self.ciudad = ciudad;
    }

    //Esto se aplica cada vez que se va a mover la hormiga.
    pub fn set_probabilidades(&mut self,matriz: &mut Vec<Vec<Conexion>>,conj_ciudades: &Vec<City>) {
        let mut sum = 0.0;
        for vecino in conj_ciudades {
            if vecino.visited == false {
                sum += matriz[self.ciudad][vecino.ciudad].visibilidad + matriz[self.ciudad][vecino.ciudad].feromona;
            }
        }

        for vecino in conj_ciudades {
            if vecino.visited == false {
                matriz[self.ciudad][vecino.ciudad].probabilidad = (matriz[self.ciudad][vecino.ciudad].visibilidad + matriz[self.ciudad][vecino.ciudad].feromona)/sum;
            }
        }

    }

}
