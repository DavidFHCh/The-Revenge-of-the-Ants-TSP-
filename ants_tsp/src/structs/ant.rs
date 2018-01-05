extern crate rand;

use structs::conexion::Conexion;
use structs::city::City;


#[derive(Clone)]
pub struct Ant{
    pub id: usize,
    pub ciudad : usize,
    pub visitados: Vec<City>,
    pub f_obj: f64,
}

impl Ant {
    pub fn new(city: usize,id1: usize) -> Self {
        Ant {
            id: id1,
            ciudad: city,
            visitados: Vec::new(),
            f_obj: 0.0,
        }
    }

    pub fn clean(&mut self) {
        self.ciudad = 0;
        self.visitados = Vec::new();
        self.f_obj = 0.0;
    }

    pub fn set_ciudad(&mut self, ciudad: usize) {
        self.ciudad = ciudad;
    }


    fn ordena(&mut self, a_ordenar: Vec<(usize,usize,&f64)>) -> Vec<(usize,usize,f64)>{
        let mut ordenado = Vec::new();
        for i in a_ordenar {
            let f = *i.2;
            if ordenado.len() == 0 {
                ordenado.insert(0,(i.clone().0,i.clone().1,f));
            } else {
                let mut j = 0;
                while f >= ordenado[j].2 {
                    if j == ordenado.len()-1 {
                        break;
                    } else {
                        j += 1;
                    }
                }

                ordenado.insert(j,(i.clone().0,i.clone().1,f));
            }
        }
        let mut ini = 0;
        for i in 1..ordenado.len() {
            if ordenado[ini] >= ordenado[i] {
                ordenado.swap(ini,i);
                ini+=1;
            } else {break;}
        }

        ordenado
    }


    pub fn mueve_hormiga(&mut self, matriz: &Vec<Vec<Conexion>>,conj_ciudades: &mut Vec<City>, select_in: f64) -> bool {
        let mut select = select_in;
        let mut selected_city: City = City::new(0);
        //let len = conj_ciudades.len();
        let mut probs_orden = Vec::new();

        for city_index in 0..conj_ciudades.len() {
            let city = &conj_ciudades[city_index];
            if city.visited == false && &matriz[self.ciudad][city.ciudad].distancia != &0.0{
                probs_orden.push((self.ciudad,city_index,&matriz[self.ciudad][city.ciudad].probabilidad));
            }
        }

        let probs_ordenadas = self.ordena(probs_orden);

        for prob in probs_ordenadas {
            //println!("{:?} {}", select, prob.2);
            if select <= prob.2 {
                conj_ciudades[prob.1].set_true_visited();
                selected_city = conj_ciudades[prob.1].clone();
                break;
            }
            select -= prob.2;
        }



        if selected_city.ciudad == 0 {

            self.ciudad = 0;
            return true
        } else {
            if matriz[self.ciudad][selected_city.ciudad].distancia == 0.0 {
                self.ciudad = 0;
                return true
            }
            self.f_obj += matriz[self.ciudad][selected_city.ciudad].distancia;
            self.ciudad = selected_city.ciudad;
            self.visitados.push(selected_city);
        }
        return false
    }


}
