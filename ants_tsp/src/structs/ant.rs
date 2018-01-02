extern crate rand;

use structs::conexion::Conexion;
use structs::city::City;
use self::rand::XorShiftRng;
use self::rand::distributions::{IndependentSample, Range};

#[derive(Clone)]
pub struct Ant{
    pub ciudad : usize,
    pub visitados: Vec<City>,
}

impl Ant {
    pub fn new(city: usize) -> Self {
        Ant {
            ciudad: city,
            visitados: Vec::new(),
        }
    }

    pub fn clean(&mut self) {
        self.ciudad = 0;
    }

    pub fn set_ciudad(&mut self, ciudad: usize) {
        self.ciudad = ciudad;
    }


    pub fn mueve_hormiga(&mut self, matriz: &mut Vec<Vec<Conexion>>,conj_ciudades: &mut Vec<City>, mut rng: XorShiftRng, aum_ferm: f64) {
        self.set_probabilidades(matriz,conj_ciudades);
        let mut sum_probs = 0.0;
        for city in conj_ciudades.clone() {
            if city.visited == false {
                sum_probs += matriz[self.ciudad][city.ciudad].probabilidad;
            }
        }

        let between = Range::new(0.0,sum_probs);
        let mut select = between.ind_sample(&mut rng);
        let mut selected_city: City = City::new(0);
        for mut city in conj_ciudades {
            if city.visited == false {
                let prb = &matriz[self.ciudad][city.ciudad].probabilidad;
                if select <= *prb {
                    city.set_true_visited();
                    selected_city = city.clone();
                    break;
                }
                select -= *prb;
            }
        }

        if selected_city.ciudad == 0 {} else {
            matriz[self.ciudad][selected_city.ciudad].feromona += aum_ferm;
            self.ciudad = selected_city.ciudad;
            self.visitados.push(selected_city);
        }
    }

    //Esto se aplica cada vez que se va a mover la hormiga.
    fn set_probabilidades(&mut self,matriz: &mut Vec<Vec<Conexion>>,conj_ciudades: &Vec<City>) {
        let mut sum = 0.0;
        for vecino in conj_ciudades {
            if vecino.visited == false {
                sum += matriz[self.ciudad][vecino.ciudad].visibilidad + matriz[self.ciudad][vecino.ciudad].feromona;
            }
        }

        for vecino in conj_ciudades {
            if vecino.visited == false {
                matriz[self.ciudad][vecino.ciudad].probabilidad = (matriz[self.ciudad][vecino.ciudad].visibilidad + matriz[self.ciudad][vecino.ciudad].feromona)/sum;
            }
        }

    }

}
