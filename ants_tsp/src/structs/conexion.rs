use std::sync::Mutex;


pub struct Conexion{
    pub ciudad1: i32,
    pub ciudad2: i32,
    pub distancia: f64,
    pub probabilidad_base: Mutex<f64>,
    pub probabilidad: Mutex<f64>,
    pub feromona: Mutex<f64>,
}


impl Conexion{

    pub fn new(ciudad_1:i32, ciudad_2:i32, distancia_1:f64, probabilidad_base_1:f64) -> Self {
        Conexion {
            ciudad1: ciudad_1,
            ciudad2: ciudad_2,
            distancia: distancia_1,
            probabilidad_base: Mutex::new(probabilidad_base_1.clone()),
            probabilidad: Mutex::new(probabilidad_base_1),
            feromona: Mutex::new(1.0),
        }
    }
/*
    pub fn actualiza_feromona_mas(&mut self) {
        *self.feromona.lock().unwrap() += AUMENTO_FEROMONA;
    }

    pub fn actualiza_feromona_menos(&mut self) {
        *self.feromona.lock().unwrap() -= DISMINUCION_FEROMONA;
    }

    pub fn actualiza_probabilidad(&mut self) {
        *self.probabilidad.lock().unwrap() = self.probabilidad_base * (*self.feromona.lock().unwrap());
    }
*/

}
