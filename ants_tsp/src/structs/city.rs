#[derive(Clone)]
pub struct City{
    pub ciudad : usize,
    pub visited : bool,
}

impl City {
    pub fn new(city: usize) -> Self {
        City {
            ciudad: city,
            visited: false,
        }
    }

    pub fn set_false_visited(&mut self) {
        self.visited = false;
    }

    pub fn set_ciudad(&mut self, ciudad: usize) {
        self.ciudad = ciudad;
    }

    pub fn set_true_visited(&mut self) {
        self.visited = true;
    }


}
