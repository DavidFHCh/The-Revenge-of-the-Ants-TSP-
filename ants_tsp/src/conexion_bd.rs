extern crate rusqlite;
use std::path::Path;
use self::rusqlite::Connection;
use structs::conexion::Conexion;
use structs::city::City;

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
pub fn get_ciudades() -> Result<(Vec<City>,Vec<Vec<Conexion>>), rusqlite::Error> {
    let path_db = Path::new("../resources/tsp.db");
    let conexion = Connection::open(path_db).unwrap();
    let mut ciudades = Vec::with_capacity(NUM_CIUDADES);

    let mut consulta2 = conexion.prepare("SELECT * FROM connections").expect("NO SE PUDO COMPLETAR LA CONEXION 2.");

    let con_it = consulta2.query_map(&[], |renglon| {
        Conexion::new(renglon.get(0),renglon.get(1),renglon.get(2),0.5)
    }).unwrap();



    let mut m_adyacencias = Vec::with_capacity(NUM_CIUDADES);

    for _v in 0..NUM_CIUDADES {
        let mut ceros = Vec::with_capacity(NUM_CIUDADES);
        for _i in 0..NUM_CIUDADES {
            ceros.push(Conexion::new(0,0,0.0,0.0));
        }
        m_adyacencias.push(ceros);
    }

    for arista in con_it {
        let aris = arista.unwrap();
        let id1 = aris.ciudad1;
        let id2 = aris.ciudad2;
        m_adyacencias[id1 as usize][id2 as usize] = aris.clone();
        m_adyacencias[id2 as usize][id1 as usize] = aris;
    }

    for i in 0..NUM_CIUDADES {
        ciudades.push(City::new(i));
    }

    let mut sum = 0.0;
    for  aristas in &mut m_adyacencias {
        sum = 0.0;
        let mut mayor = 0.0;
        for arista in aristas.clone() {
            if arista.distancia > mayor {
                mayor = arista.distancia;
            }
        }
        mayor = mayor * 1.1;

        for arista in aristas.clone() {
            sum += mayor - arista.distancia;
        }
        for arista in aristas {
            let dist = arista.distancia;
            arista.set_probabilidad_base((mayor - dist) / sum);
            arista.set_probabilidad((mayor - dist)/sum);
        }
    }




    Ok((ciudades,m_adyacencias))

}
