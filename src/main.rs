use serde::Deserialize;
use serde_json;
use std::fs;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct IPS {
    nombre: String,
    ip4: Vec<String>,
    ip6: Vec<String>,
}

#[derive(Debug)]//Los parametros comentados no se utilizan, logica implementada
struct Metricas {//si se desean utilizar, ya se extraen
    //min: f32,
    avg: f32,
    //max: f32,
    mdev: f32,   
    packet_loss: u8,
}

fn extraer_metricas(texto_salida: &str) -> Option<Metricas> {
    let linea = texto_salida.lines().find(|l| l.contains("packet loss"))?;
    let primera_mitad = linea.split("%").next()?;
    let numero = primera_mitad.split_whitespace().last()?;
    let linea_metricas = texto_salida.lines().find(|linea| linea.contains("rtt min/avg/max/mdev"))?;
    let valores = linea_metricas.split("=").nth(1)?;
    let valores_limpios = valores.trim().trim_end_matches(" ms");
    let partes: Vec<&str> = valores_limpios.split("/").collect();

    if partes.len() == 4 {
        Some(Metricas{
            //min: partes[0].parse().unwrap_or(0.0),
            avg: partes[1].parse().unwrap_or(0.0),
            //max: partes[2].parse().unwrap_or(0.0),
            mdev: partes[3].parse().unwrap_or(0.0),
            packet_loss: numero.parse::<u8>().ok()?,
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
        for ip in objetivo.ip4{
            println!("Ping a {}", ip);

            let comando = Command::new("ping")
                .arg("-c")
                .arg("4")
                .arg("-4") //Forzar IPv4
                .arg(&ip)
                .output()
                .expect("Problema al ejecutar el comando");
            let texto_salida = String::from_utf8_lossy(&comando.stdout);
            if let Some(metricas) = extraer_metricas(&texto_salida) {
                if metricas.avg > 50.00 {
                    println!("!!! Conexión lenta con IP {} ({}), tiempo {} ms", ip, objetivo.nombre, metricas.avg);
                }

                if metricas.mdev > 10.00 {
                    println!("!!! Conexion variable con IP {} ({}), variación en ping {} ms", ip, objetivo.nombre, metricas.mdev);
                }

                if metricas.packet_loss > 25 {
                    println!("!!! Conexion inestable con IP {} ({}), variación en ping {} ms", ip, objetivo.nombre, metricas.packet_loss);
                }
            } else {
                println!("PROBLEMA extrayendo datos");
            }
        }

        for ip in objetivo.ip6{
            println!("Ping a {}", ip);
            let comando = Command::new("ping")
                .arg("-c")
                .arg("4")
                .arg("-6") //Forzar IPv6
                .arg(&ip)
                .output()
                .expect("Problema al ejecutar el comando");
            let texto_salida = String::from_utf8_lossy(&comando.stdout);
            if let Some(metricas) = extraer_metricas(&texto_salida) {
                if metricas.avg > 50.00 {
                    println!("!!! Conexión lenta con IP {} ({}), tiempo {} ms", ip, objetivo.nombre, metricas.avg);
                }

                if metricas.mdev > 10.00 {
                    println!("!!! Conexion variable con IP {} ({}), variación en ping {} ms", ip, objetivo.nombre, metricas.mdev);
                }

                if metricas.packet_loss > 25 {
                    println!("!!! Conexion inestable con IP {} ({}), variación en ping {} ms", ip, objetivo.nombre, metricas.packet_loss);
                }
            } else {
                println!("PROBLEMA extrayendo datos");
            }
        }

    }
}


