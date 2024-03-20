// Define a struct to represent a Butcher Tableau
#[derive(Debug, Clone, Copy)]
pub struct ButcherTableau {
    a: [[f64; 8]; 8],
    b: [f64; 8],
    c: [f64; 8],
}

impl ButcherTableau {
    fn new(a: [[f64; 8]; 8], b: [f64; 8], c: [f64; 8]) -> Self {
        Self { a, b, c }
    }

    fn len(&self) -> usize {
        self.b.len()
    }
}

pub enum Tableau {    
    DoPri45,
    Tsit5,
}

impl Default for Tableau {
    fn default() -> Self {
        Tableau::DoPri45 // Specify Variant1 as the default value
    }
}