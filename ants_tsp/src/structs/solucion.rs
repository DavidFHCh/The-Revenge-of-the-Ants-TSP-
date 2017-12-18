pub struct Solucion {
    pub solucion: Vec<usize>,
    pub f_obj: f64,
}

impl Solucion {
    pub fn new() -> Self {
        Solucion {
            solucion: Vec::new(),
            f_obj: 0.0,
        }
    }
}
