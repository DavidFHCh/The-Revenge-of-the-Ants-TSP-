//use std::sync::Mutex;
use std::collections::VecDeque;
use structs::ant::Ant;

pub struct QueueParll{
    ants : VecDeque<Ant>,
    tiempo_evp: usize,
}

static EVAPORACION: usize = 20;

impl QueueParll{

    pub fn new() -> Self{
        QueueParll{
            ants: VecDeque::new(),
            tiempo_evp: EVAPORACION,
        }
    }

    pub fn push(&mut self,ant: Ant){
        self.ants.push_front(ant);
    }

    pub fn pop(&mut self) -> (Ant,bool) {
        let mut evapora: bool = true;
        self.tiempo_evp -= 1;
        if self.tiempo_evp == 0 {
            self.tiempo_evp = EVAPORACION;
        } else {
            evapora = false;
        }
        (self.ants.pop_front().unwrap(), evapora)
    }
}
