mod balanceador;
mod pila;
mod precedencia;
mod reglas;
mod shunting_yard;

use std::fs;

fn main() {
    println!("==========================================");
    println!(" Conversión Infix a Postfix - Shunting Yard");
    println!("==========================================");

    let contenido = fs::read_to_string("expresiones_problema3.txt")
        .expect("No fue posible abrir expresiones_problema3.txt");

    for (indice, linea) in contenido.lines().enumerate() {
        let expresion = linea.trim();

        if expresion.is_empty() {
            continue;
        }

        println!("\n------------------------------------------");
        println!("Expresión número {}", indice + 1);
        println!("------------------------------------------");

        match shunting_yard::convertir_a_postfix(expresion) {
            Ok(postfix) => {
                println!("Resultado postfix: {}", postfix);
            }
            Err(error) => {
                println!("Estado actual: {}", error);
            }
        }
    }
}