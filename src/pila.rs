pub struct Pila {
    elementos: Vec<char>,
}

impl Pila {
    pub fn nueva() -> Self {
        Self {
            elementos: Vec::new(),
        }
    }

    pub fn insertar(&mut self, valor: char) {
        self.elementos.push(valor);
    }

    pub fn extraer(&mut self) -> Option<char> {
        self.elementos.pop()
    }

    pub fn cima(&self) -> Option<&char> {
        self.elementos.last()
    }

    pub fn esta_vacia(&self) -> bool {
        self.elementos.is_empty()
    }

    pub fn mostrar(&self) {
        if self.elementos.is_empty() {
            println!("Pila: Vacía");
        } else {
            print!("Pila: ");

            for simbolo in &self.elementos {
                print!("{}", simbolo);
            }

            println!();
        }
    }
}