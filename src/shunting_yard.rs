use crate::precedencia;
use crate::reglas;

pub fn convertir_a_postfix(expresion: &str) -> Result<String, String> {
    println!("Expresión infix: {}", expresion);

    for simbolo in expresion.chars() {
        mostrar_clasificacion(simbolo);
    }

    Err(String::from(
        "La conversión Shunting Yard todavía no está implementada",
    ))
}

fn mostrar_clasificacion(simbolo: char) {
    if reglas::es_escape(simbolo) {
        println!("'{}' -> carácter de escape", simbolo);
    } else if precedencia::es_operador(simbolo) {
        println!(
            "'{}' -> operador con precedencia {}",
            simbolo,
            precedencia::obtener_precedencia(simbolo)
        );
    } else if simbolo == '(' {
        println!("'{}' -> apertura de agrupación", simbolo);
    } else if simbolo == ')' {
        println!("'{}' -> cierre de agrupación", simbolo);
    } else {
        println!("'{}' -> operando", simbolo);
    }
}