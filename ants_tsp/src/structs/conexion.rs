use std::sync::Mutex;


pub struct Conexion{
    pub ciudad1: i32,
    pub ciudad2: i32,
    pub distancia: f64,
    pub probabilidad: Mutex<f64>,
}
