use serde::Deserialize;
use serde_json;
use std::fs;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct IPS {
    nombre: String,
    ip4: Vec<String>,
    ip6: Vec<String>,
    intervalo_segundos: u8,
}

#[derive(Debug)]
struct Metricas {
    min: f32,
    avg: f32,
    max: f32,
    mdev: f32,
}

fn extraer_metricas(texto_salida: &str) -> Option<Metricas> {
    let linea_metricas = texto_salida.lines().find(|linea| linea.contains("rtt min/avg/max/mdev"))?;
    let valores = linea_metricas.split("=").nth(1)?;
    let valores_limpios = valores.trim().trim_end_matches(" ms");
    let partes: Vec<&str> = valores_limpios.split("/").collect();

    if partes.len() == 4 {
        Some(Metricas{
            min: partes[0].parse().unwrap_or(0.0),
            avg: partes[1].parse().unwrap_or(0.0),
            max: partes[2].parse().unwrap_or(0.0),
            mdev: partes[3].parse().unwrap_or(0.0),
        })
    } else {
        None
    }
}

fn main(){
    //Leer datos
    let datos = fs::read_to_string("ips.json").expect("PROBLEMA: lectura del archivo");
    //Convertir a vector
    let objetivos: Vec<IPS> = serde_json::from_str(&datos)
    .expect("PROBLEMA: con los datos");

    println!("============== Iniciando KOALA =================");

    for objetivo in objetivos{
        println!("Prueba a {}", objetivo.nombre);

        for ip in objetivo.ip4{
            println!("Ping a {}", ip);

            let comando = Command::new("ping")
                .arg("-c")
                .arg("4")
                .arg("-4") //Forzar IPv4
                .arg(ip)
                .output()
                .expect("Problema al ejecutar el comando");

            let texto_salida = String::from_utf8_lossy(&comando.stdout);
            if let Some(metricas) = extraer_metricas(&texto_salida) {
                println!("Exito, media {} ms", metricas.avg);
            } else {
                println!("PROBLEMA extrayendo datos");
            }
        }
    }
}


