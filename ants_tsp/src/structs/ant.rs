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
        let mut probs = 0.0;
        for city in ciudades_visitar {
            probs += matriz[self.ciudad][city].probabilidad.lock().unwrap();
        }
        //tienes que agregar el rng
        matriz[self.ciudad]
        (15,0.0)
    }
}
