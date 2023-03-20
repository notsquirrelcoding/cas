use crate::expr::Function;

use super::polynomial::PolyExpr;

pub enum BaseFunc {
    Ln,
    Log(f64),
    Poly(PolyExpr),
    Rational(PolyExpr, PolyExpr),
    Sin,
    Arcsin,
    Cos,
    Arccos,
    Tan,
    Arctan,
    Exp(f64),
    Power(f64),
}

impl BaseFunc {
    pub fn eval(&self, x: f64) -> f64 {
        match self {
            BaseFunc::Ln => x.ln(),
            BaseFunc::Log(base) => x.log(*base),
            BaseFunc::Poly(expr) => expr.eval(x),
            BaseFunc::Rational(p, q) => p.eval(x) / q.eval(x),
            BaseFunc::Sin => x.sin(),
            BaseFunc::Arcsin => x.asin(),
            BaseFunc::Cos => x.cos(),
            BaseFunc::Arccos => x.acos(),
            BaseFunc::Tan => x.tan(),
            BaseFunc::Arctan => x.atan(),
            BaseFunc::Exp(base) => base.powf(x),
            BaseFunc::Power(power) => x.powf(*power),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{E, PI};

    use crate::expr::{
        base_func::BaseFunc,
        polynomial::{poly, PolyExpr},
    };

    #[test]
    fn test_eval() {
        // painstakingly try every combination

        assert_eq!(BaseFunc::Arccos.eval(1.0), 0.0);
        assert_eq!(BaseFunc::Arcsin.eval(1.0), PI / 2.0);
        assert_eq!(BaseFunc::Arctan.eval(1.0), PI / 4.0);
        assert_eq!(BaseFunc::Cos.eval(PI), -1.0);
        assert_eq!(BaseFunc::Exp(5.0).eval(1.23), 7.239908964061695);
        assert_eq!(BaseFunc::Ln.eval(E), 1.0);
        assert_eq!(BaseFunc::Log(4.0).eval(24.234543), 2.2994964860993843);
        assert_eq!(
            BaseFunc::Poly(poly!(2.0, -24.0, 314.0, 4.23, 536.0, 0.0, 0.0, 24.0)).eval(4.2334),
            379585.7103670243
        );
        assert_eq!(BaseFunc::Power(0.5).eval(4.0), 2.0);
        assert_eq!(BaseFunc::Rational(poly!(3.0, 1.0, 0.0), poly!(6.4, 2.4, 2.7)).eval(3.0), 0.0);
        assert_eq!(BaseFunc::Arccos.eval(1.0), 0.0);
        assert_eq!(BaseFunc::Arccos.eval(1.0), 0.0);
    }
}
