use serde::Deserialize;
use serde_json;
use std::fs;

#[derive(Deserialize, Debug)]
struct IPS {
    nombre: String,
    ip4: Vec<String>,
    ip6: Vec<String>,
    intervalo_segundos: u8,
}

fn main(){
    //Leer datos
    let datos = fs::read_to_string("ips.json").expect("PROBLEMA: lectura del archivo");
    let objetivos: Vec<IPS> = serde_json::from_str(&datos)
    .expect("PROBLEMA: con los datos");

    //Muestreo lectura
    for objetivo in objetivos{
        println!("=============Lectura de Datos============");
        println!("Nombre: {}\nIntervalo: {}",objetivo.nombre, objetivo.intervalo_segundos);
        println!("IPv4: {} y {}", objetivo.ip4[0], objetivo.ip4[1]);
        println!("IPv6: {} y {}", objetivo.ip6[0], objetivo.ip6[1]);
    }

    println!("=================FIN======================");
}


