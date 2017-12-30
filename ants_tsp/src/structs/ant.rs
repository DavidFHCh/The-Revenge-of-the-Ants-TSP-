extern crate rand;

use structs::conexion::Conexion;
use std::sync::{Arc};
use self::rand::{XorShiftRng, SeedableRng, Rng};
use self::rand::distributions::{IndependentSample, Range};

#[derive(Clone)]
pub struct Ant{
    pub ciudad : usize,
}

impl Ant {
    pub fn new(city: usize) -> Self {
        Ant {
            ciudad: city,
        }
    }

    pub fn clean(&mut self) {
        self.ciudad = 0;
    }

    pub fn set_ciudad(&mut self, ciudad: usize) {
        self.ciudad = ciudad;
    }


}
