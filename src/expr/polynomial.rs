use core::prelude;
use std::{fmt, collections::VecDeque};

use crate::expr::join::JoinType;

use super::{base_func::BaseFunc, join::{Join, JoinFuncError}, Function};

#[derive(Debug, Clone)]
pub struct PolyExpr {
    degree: usize,
    pub coefficients: Vec<f64>,
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

            let monomial = constant.join(power, JoinType::Product)?;

            vec.push(monomial);
        }

        Ok(vec)
    }

    pub fn combine_terms(self, other: PolyExpr) -> PolyExpr {
        // The degree of the smaller polynomial
        let (smaller, larger) = match self.degree.cmp(&other.degree) {
            std::cmp::Ordering::Less => (self.clone(), other.clone()),
            std::cmp::Ordering::Equal => return PolyExpr::new(self.coefficients.iter().zip(other.coefficients).map(|(a, b)| { a + b }).collect()),
            std::cmp::Ordering::Greater => (other.clone(), self.clone()),
        };

        let smaller_degree = smaller.degree;

        let mut new_coefficients = Vec::new();

        // loop and check for like terms. if so add to queue

        for i in (0..=smaller_degree).rev() {
            new_coefficients.push(smaller.coefficients[smaller_degree + i - 2] + larger.coefficients[i + 1]);
        }

        // Add the remaining terms of the larger polynomial
        new_coefficients.append(&mut larger.coefficients[..(smaller_degree)].to_vec());

        new_coefficients.reverse();

        PolyExpr::new(new_coefficients)
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

impl fmt::Display for PolyExpr {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut string = String::new();

        for i in (0..(self.degree)).rev() {
            if self.coefficients[self.degree - i] == 0.0 {
                continue;
            }

            let sign = match (self.coefficients[self.degree - i] as i32).signum() {
                1 => {
                    String::from("+")
                },
                -1 => {
                    String::from("-")
                },
                _ => unreachable!()
            };

            string.push_str(&format!(" {}{}x^{} ", sign, self.coefficients[self.degree - i].abs(), i))
            
        }


        write!(f, "{}", string)
    }
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
