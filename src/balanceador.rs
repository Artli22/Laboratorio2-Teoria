use crate::pila::Pila;

pub fn verificar_balanceo(expresion: &str) -> bool {

    let mut pila = Pila::nueva();

    println!("\nExpresión:");
    println!("{}", expresion);
    println!();

    for caracter in expresion.chars() {

        if es_apertura(caracter) {

            println!("Leer '{}'", caracter);
            println!("Push {}", caracter);

            pila.insertar(caracter);
            pila.mostrar();

            println!();

        } else if es_cierre(caracter) {

            println!("Leer '{}'", caracter);

            if pila.esta_vacia() {

                println!("Error: no existe símbolo de apertura.");

                return false;
            }

            let ultimo = pila.extraer().unwrap();

            println!("Pop {}", ultimo);

            if ultimo != simbolo_correspondiente(caracter) {

                println!("Error: símbolos incompatibles.");

                return false;
            }

            pila.mostrar();

            println!();
        }
    }

    if pila.esta_vacia() {

        true

    } else {

        println!("Error: quedaron símbolos sin cerrar.");

        false
    }
}

fn es_apertura(c: char) -> bool {

    c == '(' || c == '[' || c == '{'
}

fn es_cierre(c: char) -> bool {

    c == ')' || c == ']' || c == '}'
}

fn simbolo_correspondiente(c: char) -> char {

    match c {

        ')' => '(',

        ']' => '[',

        '}' => '{',

        _ => ' '
    }
}