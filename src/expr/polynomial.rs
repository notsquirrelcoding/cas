use super::Function;

pub struct PolyExpr {
    degree: usize,
    coefficients: Vec<f64>
}

impl PolyExpr {
    pub fn new(coefficients: Vec<f64>) -> Self {
        let degree = coefficients.len() - 1;

        Self {
            degree,
            coefficients,
        }
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
}struct Symbol {
    ident: char
}

impl Symbol {
    pub fn new(ident: char) -> Self {
        Symbol { ident }
    }
}

macro_rules! poly {    
    ( $( $x:literal ),* ) => {
        {
            let mut temp_vec = Vec::new();

            $(
                temp_vec.push($x);
            )*

            PolyExpr::new(temp_vec)
        }
    };
}

pub(crate) use poly;
