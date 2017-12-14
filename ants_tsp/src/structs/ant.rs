pub struct Ant{
    ciudad : usize,
}


impl Ant{
    pub fn new(id :usize) -> Self{
        Ant {
            ciudad: id,
        }
    }
}
