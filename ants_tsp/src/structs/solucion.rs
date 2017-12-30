pub struct Solucion {
    pub num_sol: usize,
    pub solucion: Vec<usize>,
    pub f_obj: f64,
}

impl Solucion {
    pub fn new(id: usize) -> Self {
        Solucion {
            num_sol: id,
            solucion: Vec::new(),
            f_obj: 0.0,
        }
    }
}
