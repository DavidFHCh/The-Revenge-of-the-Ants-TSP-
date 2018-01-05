#[derive(Clone)]
pub struct Conexion{
    pub ciudad1: i32,
    pub ciudad2: i32,
    pub distancia: f64,
    pub probabilidad: f64,
    pub visibilidad: f64,
    pub feromona: f64,
}


impl Conexion{

    pub fn new(ciudad_1:i32, ciudad_2:i32, distancia_1:f64, probabilidad_base_1:f64) -> Self {
        Conexion {
            ciudad1: ciudad_1,
            ciudad2: ciudad_2,
            distancia: distancia_1,
            probabilidad: probabilidad_base_1,
            visibilidad: 0.0,
            feromona: 0.0,
        }
    }

    pub fn set_distancia(&mut self, dist:f64) {
        self.distancia = dist;
    }

    pub fn set_probabilidad_base(&mut self, prob:f64) {
        self.probabilidad = prob;
    }

    pub fn set_probabilidad(&mut self, prob:f64) {
        self.probabilidad = prob;
    }

    pub fn set_feromona(&mut self, feromona: f64) {
        self.feromona = feromona;
    }

}
