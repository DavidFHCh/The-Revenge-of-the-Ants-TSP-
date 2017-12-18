use structs::conexion::Conexion;
use std::sync::{Arc};

pub struct Ant{
    pub ciudad : usize,
}


impl Ant{
    pub fn new(id :usize) -> Self{
        Ant {
            ciudad: id,
        }
    }

    pub fn muevete(&mut self,matriz: &Arc<Vec<Vec<Conexion>>>, ciudades_visitar:&Vec<usize>) -> (usize,f64) {
        self.ciudad = 15;
        (0,0.0)
    }
}
