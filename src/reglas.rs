pub fn es_escape(simbolo: char) -> bool {
    simbolo == '\\'
}

pub fn es_cuantificador(simbolo: char) -> bool {
    matches!(simbolo, '*' | '+' | '?')
}

pub fn requiere_conversion(simbolo: char) -> bool {
    matches!(simbolo, '+' | '?')
}

pub fn descripcion_regla(simbolo: char) -> &'static str {
    match simbolo {
        '\\' => "Carácter de escape",
        '*' => "Cerradura de Kleene: cero o más repeticiones",
        '+' => "Cerradura positiva: una o más repeticiones",
        '?' => "Elemento opcional: cero o una aparición",
        _ => "No es un símbolo reservado",
    }
}