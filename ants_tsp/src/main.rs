//fn main() {
//    println!("Hello, world!");
//}
#![feature(vec_remove_item)]
extern crate threadpool;
extern crate num_cpus;
extern crate ants_tsp;
extern crate rand;
extern crate config;

use ants_tsp as ants;
//use std::sync::{Arc};
use ants::conexion_bd::get_ciudades;
use rand::{XorShiftRng, SeedableRng, Rng};
use ants_tsp::structs::conexion::Conexion;
use ants_tsp::structs::ant::Ant;
use ants_tsp::structs::city::City;
use ants_tsp::structs::solucion::Solucion;
use rand::distributions::{IndependentSample, Range};
use config::{Config, File, FileFormat, Value};



static RECORRIDOS: usize = 400;
static NUM_HORMIGAS: usize = 20;
static AUMENTO_FEROMONA: f64 = 0.1;
static DISMINUCION_FEROMONA: f64 = 0.85;


fn to_usize_vec(values: Vec<Value>) -> Vec<usize> {
    let mut v = Vec::with_capacity(values.len());
    for vs in values.clone() {
        v.push(vs.into_int().expect("Error convirtiendo a i64") as usize);
    }
    v
}

fn to_u32_vec(values: Vec<Value>) -> Vec<u32> {
    let mut v = Vec::with_capacity(values.len());
    for vs in values.clone() {
        v.push(vs.into_int().expect("Error convirtiendo a i64") as u32);
    }
    v
}

fn set_all_false(conj_ciudades:&mut Vec<City>) {
    for city in conj_ciudades {
        city.set_false_visited();
    }
}

fn set_visibility(matriz: &mut Vec<Vec<Conexion>>, conj_ciudades: &Vec<City>) {
    let mut sum = 0.0;
    for city in conj_ciudades {
        for city_1 in conj_ciudades {
            sum += matriz[city.ciudad][city_1.ciudad].distancia;
        }
    }

    for city in conj_ciudades {
        for city_1 in conj_ciudades {
            matriz[city.ciudad][city_1.ciudad].visibilidad = matriz[city.ciudad][city_1.ciudad].distancia/sum;
        }
    }
}

fn main() {
    //let mut soluciones = Vec::new();
    let mut c = Config::new();
    let ciudades_matriz = get_ciudades().unwrap();
    let mut matriz = ciudades_matriz.1;
    c.merge(File::new("Ajustes", FileFormat::Toml).required(true)).expect("NO HAY ARCHIVO DE CONFIGURACION 'Ajustes.toml'");

    let semillas: Vec<u32> = to_u32_vec(c.get_array("seeds").expect("No hay lista de semillas declarada en Ajustes.toml"));
    let conjunto_ciudades = to_usize_vec(c.get_array("ciudad_ids").expect("No hay lista de ids de ciudades declarada en Ajustes.toml"));
    //println!("{}", conjunto_ciudades.len());
    //let conjunto_ciudades_1 = conjunto_ciudades.unwrap();
    let mut ciudades_a_visitar = Vec::new();
    for ciudad in &conjunto_ciudades {
        ciudades_a_visitar.push(City::new(*ciudad));
    }

    let mut hormigas = Vec::new();
    for _num in 0..NUM_HORMIGAS {
        let hormiga = Ant::new(0);

        hormigas.push(hormiga);
    }

    println!("{:?}", conjunto_ciudades);

    set_visibility(&mut matriz,&ciudades_a_visitar);
    for semilla in semillas {
        let mut matriz_ind = matriz.clone();
        let seed = [semilla, semilla*3, semilla*5, semilla*7];
        let mut rng: XorShiftRng = SeedableRng::from_seed(seed);


        let mut solucion = Solucion::new(semilla as usize);
        for i in 0..RECORRIDOS {
            for mut hormiga in hormigas.clone() {
                set_all_false(&mut ciudades_a_visitar);
                let leng = ciudades_a_visitar.len();
                if hormiga.visitados.len() == 0 {
                    let ciudad_aux_rang = Range::new(0,leng);
                    let ind = ciudad_aux_rang.ind_sample(&mut rng);
                    let ciudad_aux = &mut ciudades_a_visitar[ind];
                    ciudad_aux.set_true_visited();

                    hormiga.set_ciudad(ciudad_aux.ciudad);
                    hormiga.visitados.push(ciudad_aux.clone());
                }
                //todo lo del movimiento de la hormiga
                while hormiga.visitados.len() <  leng{
                    if hormiga.ciudad == 0 {
                        break;
                    }

                    let mut sum = 0.0;
                    //actualizacion de probabilidad de elegir la siguiente ciudad
                    for vecino in &ciudades_a_visitar {
                        if vecino.visited == false {
                            sum += matriz_ind[hormiga.ciudad][vecino.ciudad].visibilidad + matriz_ind[hormiga.ciudad][vecino.ciudad].feromona;
                        }
                    }
                    for vecino in &ciudades_a_visitar {
                        if vecino.visited == false {
                            matriz_ind[hormiga.ciudad][vecino.ciudad].probabilidad = (matriz_ind[hormiga.ciudad][vecino.ciudad].visibilidad + matriz_ind[hormiga.ciudad][vecino.ciudad].feromona)/sum;
                        }
                    }


                    let sum_probs = 1.0;
                    let between = Range::new(0.0,sum_probs);
                    let selection = between.ind_sample(&mut rng);
                    hormiga.mueve_hormiga(&mut matriz_ind, &mut ciudades_a_visitar,AUMENTO_FEROMONA,selection);
                    //println!("{} {}", leng, hormiga.visitados.len());
                }

                if hormiga.visitados.len() == ciudades_a_visitar.len() {
                    for ci in 0..hormiga.visitados.len()-1 {
                        println!("Factible yes", );
                        print!("[", );
                        for visit in &hormiga.visitados {
                            print!("{:?},", visit.ciudad);
                        }
                        println!("]", );
                        matriz_ind[hormiga.visitados[ci].ciudad][hormiga.visitados[ci+1].ciudad].feromona += AUMENTO_FEROMONA;
                    }
                }

            }
            for cons in &mut matriz_ind {
                for mut con in cons {
                    con.feromona = con.feromona * DISMINUCION_FEROMONA;
                }
            }

        }
    }


}
