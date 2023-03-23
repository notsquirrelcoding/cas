mod expr;
use expr::{
    join::{Join, JoinType}, Symbol, polynomial::{PolyExpr, poly},
};

use crate::expr::base_func::BaseFunc;

fn main() {
    let p = poly!(5.0, -4.0, 3.14, 5.0);
    let q = poly!(0.0, 3.0, 5.0);
    println!("{:?}", p.coefficients);
    println!("{}", p.combine_terms(q));
}

enum Form {
    Linear,
    Quadratic,
    Exponential,
    Trigonometric
}

struct CAS {
    lhs: Join,
    rhs: Join,
    symbol: Symbol
}

impl CAS {


    /// Creates a new CAS instance
    pub fn new(lhs: Join, rhs: Join, symbol: Symbol) -> CAS {
        Self {
            lhs, rhs, symbol
        }
    }

    /// Solves for the symbol in the `symbol` field
    pub fn solve(&mut self) -> Join {



        todo!()
    }

}