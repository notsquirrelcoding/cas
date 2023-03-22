mod expr;
use expr::{
    join::{Join, JoinType}, Symbol, polynomial::{PolyExpr, poly},
};

use crate::expr::base_func::BaseFunc;

fn main() {
    let lhs = Join::new(BaseFunc::Poly(poly!(10.0, 0.0)));
    let rhs = Join::new(BaseFunc::Constant(5.0));

    let x_symbol = Symbol::new('x');

    let mut cas = CAS::new(lhs, rhs, x_symbol);

    cas.solve();


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