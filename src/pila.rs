pub struct Pila {
    elementos: Vec<char>,
}

impl Pila {
    pub fn nueva() -> Self {
        Self {
            elementos: Vec::new(),
        }
    }

    pub fn insertar(&mut self, valor: char) {}

    pub fn extraer(&mut self) -> Option<char> {
        None
    }

    pub fn cima(&self) -> Option<&char> {
        None
    }

    pub fn esta_vacia(&self) -> bool {
        true
    }
}