mod pila;
mod balanceador;

use std::fs;

fn main() {

    println!("==================================");
    println!(" Balanceador de Expresiones Infix ");
    println!("==================================");

    let contenido = fs::read_to_string("expresiones.txt")
        .expect("No fue posible abrir expresiones.txt");

    for linea in contenido.lines() {

        let resultado = balanceador::verificar_balanceo(linea);

        if resultado {

            println!("Resultado: Balanceada");

        } else {

            println!("Resultado: No balanceada");
        }

        println!("--------------------------------------");
    }
}