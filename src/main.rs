mod expr;
use expr::{polynomial::{PolyExpr, poly}, Symbol};

use crate::expr::Function;

fn main() {
    //                          x^3   x^2  x^1  x^0
    let polyn = poly!(1.0, 24.0, 0.0, 4.0);


    let x = 4.0;

    println!("f({x}) = {}", polyn.eval(x));
}

enum Form {
    Sin(Symbol),
    Cos(Symbol),
    Log(Symbol),
    Poly(PolyExpr),
    Exp(Symbol),
    Root(Symbol),
    Rational(Symbol),
}


// + * f(g(x))

enum Join {
    Addition()
}

impl Form {

}