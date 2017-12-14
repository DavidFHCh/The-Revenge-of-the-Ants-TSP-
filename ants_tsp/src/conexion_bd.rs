extern crate rusqlite;
use std::path::Path;
use self::rusqlite::Connection;
use structs::conexion::Conexion;
use std::sync::{Arc,Mutex};

const NUM_CIUDADES: usize = 1093;

/// Regresa un Result que contiene un vector de ciudades o un error de rusqlite.
///
/// # Examples
/// ```
/// extern crate recocido_simulado;
/// use recocido_simulado::conexion_bd::get_ciudades;
/// fn main() {
///     let ciudades = get_ciudades().unwrap();
///     assert_eq!(ciudades.len(),1093);
/// }
/// ```
pub fn get_ciudades() -> Result<(Vec<usize>,Arc<Vec<Vec<Conexion>>>), rusqlite::Error> {
    let path_db = Path::new("../resources/tsp.db");
    let conexion = Connection::open(path_db).unwrap();
    let mut ciudades = Vec::with_capacity(NUM_CIUDADES);

    let mut consulta2 = conexion.prepare("SELECT * FROM connections").expect("NO SE PUDO COMPLETAR LA CONEXION 2.");

    let con_it = consulta2.query_map(&[], |renglon| {
        Conexion{
            ciudad1: renglon.get(0),
            ciudad2: renglon.get(1),
            distancia: renglon.get(2),
            probabilidad: Mutex::new(1.0),
        }
    }).unwrap();



    let mut m_adyacencias = Vec::with_capacity(NUM_CIUDADES);

    for _v in 0..NUM_CIUDADES {
        let mut ceros = Vec::with_capacity(NUM_CIUDADES);
        for _i in 0..NUM_CIUDADES {
            ceros.push(
                Conexion{
                ciudad1: 0,
                ciudad2: 0,
                distancia: 0.0,
                probabilidad: Mutex::new(0.0),
            });
        }
        m_adyacencias.push(ceros);
    }

    for arista in con_it {
        let aris = arista.unwrap();
        let id1 = aris.ciudad1;
        let id2 = aris.ciudad2;
        m_adyacencias[id1 as usize][id2 as usize] = aris;
    }

    let mut consulta3 = conexion.prepare("SELECT * FROM connections").expect("NO SE PUDO COMPLETAR LA CONEXION 2.");

    let con_it_2 = consulta3.query_map(&[], |renglon| {
        Conexion{
            ciudad1: renglon.get(0),
            ciudad2: renglon.get(1),
            distancia: renglon.get(2),
            probabilidad: Mutex::new(1.0),
        }
    }).unwrap();

    for arista in con_it_2 {
        let aris = arista.unwrap();
        let id1 = aris.ciudad1;
        let id2 = aris.ciudad2;
        m_adyacencias[id2 as usize][id1 as usize] = aris;
    }

    for i in 0..NUM_CIUDADES {
        ciudades.push(i);
    }

    Ok((ciudades,Arc::new(m_adyacencias)))

}


//https://www.reddit.com/r/rust/comments/4cxpym/sharing_a_mutable_vector_across_multiple_threads/?st=jaur6k4q&sh=ccdbea33
