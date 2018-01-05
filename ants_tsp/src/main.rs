
extern crate threadpool;
extern crate num_cpus;
extern crate ants_tsp;
extern crate rand;
extern crate config;

use ants_tsp as ants;
//use std::sync::{Arc};
use std::env;
use ants::conexion_bd::get_ciudades;
use rand::{XorShiftRng, SeedableRng};
use ants_tsp::structs::conexion::Conexion;
use ants_tsp::structs::ant::Ant;
use ants_tsp::structs::city::City;
use ants_tsp::structs::solucion::Solucion;
use rand::distributions::{IndependentSample, Range};
use config::{Config, File, FileFormat, Value};



static RECORRIDOS: usize = 2000;
static NUM_HORMIGAS: usize = 20;
static AUMENTO_FEROMONA: f64 = 0.02;
static DISMINUCION_FEROMONA: f64 = 0.8;
static FEROMONA_INICIAL: f64 = 0.25;


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
    let max_dist = 4982205.69 + 1000000.0;
    let mut sum = 0.0;
    for city in conj_ciudades {
        for city_1 in conj_ciudades {
            sum += matriz[city.ciudad][city_1.ciudad].distancia;
        }
    }

    for city in conj_ciudades {
        for city_1 in conj_ciudades {
            matriz[city.ciudad][city_1.ciudad].visibilidad = (max_dist - matriz[city.ciudad][city_1.ciudad].distancia)/sum;
        }
    }
}

fn main() {
    let mut solucion_mejor_global = Solucion::new(0);
    let args: Vec<String> = env::args().collect();
    let mut tipo_ayuda = 0;
    let mut inicial = 0;

    if args.len() > 1 {
        if args[1] == "ciudadInicial" {
            tipo_ayuda = 1;
        }
    }
    let mut c = Config::new();
    let ciudades_matriz = get_ciudades().unwrap();
    let mut matriz = ciudades_matriz.1;
    c.merge(File::new("Ajustes", FileFormat::Toml).required(true)).expect("NO HAY ARCHIVO DE CONFIGURACION 'Ajustes.toml'");

    let semillas: Vec<u32> = to_u32_vec(c.get_array("seeds").expect("No hay lista de semillas declarada en Ajustes.toml"));
    let conjunto_ciudades = to_usize_vec(c.get_array("ciudad_ids").expect("No hay lista de ids de ciudades declarada en Ajustes.toml"));
    let camino_inicial = to_usize_vec(c.get_array("camino_incial").expect("No hay lista de ciudades iniciales declarada en Ajustes.toml"));

    if camino_inicial.len() != 1 {
        tipo_ayuda = 2;
    }
    //println!("{}", conjunto_ciudades.len());
    //let conjunto_ciudades_1 = conjunto_ciudades.unwrap();
    let mut ciudades_a_visitar = Vec::new();
    for ciudad in &conjunto_ciudades {
        ciudades_a_visitar.push(City::new(*ciudad));
    }

    let mut hormigas = Vec::new();
    for num in 0..NUM_HORMIGAS {
        let hormiga = Ant::new(0,num);

        hormigas.push(hormiga);
    }

    println!("{:?}", conjunto_ciudades);

    set_visibility(&mut matriz,&ciudades_a_visitar);
    if tipo_ayuda == 2 {
        for i in 0..camino_inicial.len()-1 {
            matriz[camino_inicial[i]][camino_inicial[i+1]].feromona += FEROMONA_INICIAL;
            matriz[camino_inicial[i+1]][camino_inicial[i]].feromona += FEROMONA_INICIAL;
        }
        inicial = camino_inicial[0].clone();
    }
    for semilla in semillas {
         println!("----------------------------------------\n\n----------------------------------------\nSemilla: {}", semilla);
        let mut matriz_ind = matriz.clone();
        let seed = [semilla, semilla*3, semilla*5, semilla*7];
        let mut rng: XorShiftRng = SeedableRng::from_seed(seed);

        let mut solucion = Solucion::new(semilla as usize);
        for _i in 0..RECORRIDOS {
            for mut hormiga in hormigas.clone() {
                set_all_false(&mut ciudades_a_visitar);
                let leng = ciudades_a_visitar.len();
                if hormiga.visitados.len() == 0 {
                    if tipo_ayuda == 0 {
                        let ciudad_aux_rang = Range::new(0,leng);
                        let ind = ciudad_aux_rang.ind_sample(&mut rng);
                        let ciudad_aux = &mut ciudades_a_visitar[ind];
                        ciudad_aux.set_true_visited();

                        hormiga.set_ciudad(ciudad_aux.ciudad);
                        hormiga.visitados.push(ciudad_aux.clone());
                    } else {
                        let mut ciudad_ind = 0;
                        let mut ciudad_ini;
                        if tipo_ayuda == 1 {
                            ciudad_ini = args[2].parse::<usize>().unwrap();
                        } else {
                            ciudad_ini = inicial.clone();
                        }
                        for ciudad1_index in 0..ciudades_a_visitar.len() {
                            if ciudades_a_visitar[ciudad1_index].ciudad == ciudad_ini {
                                ciudad_ind = ciudad1_index.clone();
                                break;
                            }
                        }
                        let ciudad_aux =  &mut ciudades_a_visitar[ciudad_ind];
                        ciudad_aux.set_true_visited();

                        hormiga.set_ciudad(ciudad_aux.ciudad);
                        hormiga.visitados.push(ciudad_aux.clone());
                    }

                }
                let mut termina_mal: bool;
                //todo lo del movimiento de la hormiga
                while hormiga.visitados.len() <  leng{
                    if hormiga.ciudad == 0 {
                        break;
                    }

                    let mut sum = 0.0;
                    //actualizacion de probabilidad de elegir la siguiente ciudad
                    for vecino in &ciudades_a_visitar {
                        if vecino.visited == false && &matriz[hormiga.ciudad][vecino.ciudad].distancia != &0.0{
                            sum += matriz_ind[hormiga.ciudad][vecino.ciudad].visibilidad + matriz_ind[hormiga.ciudad][vecino.ciudad].feromona;
                        }
                    }
                    for vecino in &ciudades_a_visitar {
                        if vecino.visited == false && &matriz[hormiga.ciudad][vecino.ciudad].distancia != &0.0{
                            matriz_ind[hormiga.ciudad][vecino.ciudad].probabilidad = (matriz_ind[hormiga.ciudad][vecino.ciudad].visibilidad + matriz_ind[hormiga.ciudad][vecino.ciudad].feromona)/sum;
                            //println!("{:?}", matriz_ind[hormiga.ciudad][vecino.ciudad].probabilidad);
                        }
                    }


                    let sum_probs = 1.0;
                    let between = Range::new(0.0,sum_probs);
                    let selection = between.ind_sample(&mut rng);
                    termina_mal = hormiga.mueve_hormiga(&mut matriz_ind, &mut ciudades_a_visitar,selection);
                    if termina_mal {
                        break;
                    }
                    //println!("{} {}", leng, hormiga.visitados.len());
                }//termina hormiga

                if hormiga.visitados.len() == ciudades_a_visitar.len() {
                    for ci in 0..hormiga.visitados.len()-1 {
                        matriz_ind[hormiga.visitados[ci].ciudad][hormiga.visitados[ci+1].ciudad].feromona += AUMENTO_FEROMONA;
                        matriz_ind[hormiga.visitados[ci+1].ciudad][hormiga.visitados[ci].ciudad].feromona += AUMENTO_FEROMONA;
                    }

                    if hormiga.f_obj < solucion.f_obj {
                        /*
                        println!("----------------------------------", );
                        println!("{:?}", hormiga.f_obj);
                        println!("{:?}",args);
                        println!("{:?}", hormiga.id);
                        println!("Factible yes", );
                        print!("[", );
                        */
                        let mut sol = Vec::new();
                        for visit in &hormiga.visitados {
                            //print!("{:?},", visit.ciudad);
                            sol.push(visit.ciudad.clone());
                        }
                        //println!("]", );
                        solucion.f_obj = hormiga.f_obj;
                        solucion.solucion = sol;
                    }

                }
                hormiga.clean();
            }// termina hormigas
            for cons in &mut matriz_ind {
                for con in cons {
                    con.feromona = con.feromona * DISMINUCION_FEROMONA;
                }
            }

        }//termina RECORRIDOS
        let mut fact = true;
        if solucion.solucion.len() == 0 {
            fact = false;
        }
        println!("----------------------------------", );
        println!("Semilla: {:?}", solucion.num_sol);
        println!("F_OBJ: {:?}", solucion.f_obj);
        println!("Factible: {}", fact);
        println!("{:?}", solucion.solucion );

        if solucion.f_obj < solucion_mejor_global.f_obj {
            solucion_mejor_global = solucion.clone();
        }
        solucion.clean();
    }//termina semillas
    println!("----------------------------------------\n----------------------------------------\n----------------------------------------\n----------------------------------------", );
    println!("Mejor semilla: {:?}", solucion_mejor_global.num_sol);
    println!("Funcion objeto: {:?}", solucion_mejor_global.f_obj);
    println!("{:?}", solucion_mejor_global.solucion);
}
