use crate::expr::join::JoinType;

use super::{base_func::BaseFunc, join::{Join, JoinFuncError}, Function};

#[derive(Debug, Clone)]
pub struct PolyExpr {
    degree: usize,
    coefficients: Vec<f64>,
}

impl PolyExpr {
    pub fn new(coefficients: Vec<f64>) -> Self {
        let degree = coefficients.len() - 1;

        Self {
            degree,
            coefficients,
        }
    }

    fn destructure(&self) -> Result<Vec<Join>, JoinFuncError> {
        let mut vec = Vec::with_capacity(self.degree);

        for deg in (0..self.degree).rev() {
            let constant = Join::new(BaseFunc::Constant(
                self.coefficients[self.coefficients.len() - deg],
            ));

            let power = Join::new(BaseFunc::Power(deg as f64));

            vec.push(constant.join(power, JoinType::Product));
        }
        
        vec
    }

    fn combine_terms(&mut self) {

        // let

        // loop and check for like terms. if so add to queue

        // combine like terms and remove old ones
    }
}

impl Function for PolyExpr {
    fn eval(&self, x: f64) -> f64 {
        let mut power = self.degree;

        let mut result = 0.0;

        for i in &self.coefficients {
            println!("{i}x^{power}");
            result += i * x.powi(power as i32);

            power = power.saturating_sub(1);
        }

        result
    }
}
struct Symbol {
    ident: char,
}

impl Symbol {
    pub fn new(ident: char) -> Self {
        Symbol { ident }
    }
}

macro_rules! poly {

    ( $( $x:literal ),* ) => {
        {
            PolyExpr::new(vec![$( $x, )* ])
        }
    };
}

pub(crate) use poly;
