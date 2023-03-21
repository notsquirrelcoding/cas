use self::polynomial::PolyExpr;

pub mod base_func;
pub mod join;
pub mod polynomial;

enum Expression {
    Polynomial(Box<PolyExpr>),
    Transcedental,
}

pub trait Function {
    fn eval(&self, x: f64) -> f64;
}

pub struct Symbol {
    ident: char,
    val: Option<f64>,
}

impl Symbol {
    pub fn new(ident: char) -> Self {
        Symbol { ident, val: None }
    }
}
