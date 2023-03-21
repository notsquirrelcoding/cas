use super::base_func::BaseFunc;

#[derive(Debug, Clone)]
/// A struct that represents the sum product, and composition of two functions. 
/// It is a recursive structure so any elementary function can be made using this.
pub struct Join {
    lhs: Option<Box<Join>>,
    rhs: Option<Box<Join>>,
    join_type: JoinType,
    base: Option<BaseFunc>,
}

impl Join {
    pub fn new(f: BaseFunc) -> Join {
        Join {
            lhs: None,
            rhs: None,
            join_type: JoinType::Base,
            base: Some(f),
        }
    }

    /// Evaluates the sum, product, and composition of two functions using recursion.
    pub fn eval(&self, x: f64) -> f64 {
        // Base case: there exists a `base` function.
        if let Some(func) = &self.base {
            return func.eval(x);
        }

        let lhs_result = self.lhs.clone().unwrap().eval(x);
        let rhs_result = self.rhs.clone().unwrap().eval(x);

        // Compose the results together
        match self.join_type {
            JoinType::Base => unreachable!(),
            JoinType::Sum => lhs_result + rhs_result,
            JoinType::Product => lhs_result * rhs_result,
            // Evaluate the rhs_result and plug it into the first function.
            JoinType::Composition => self.lhs.clone().unwrap().eval(rhs_result),
        }
    }

    /// Joins two joins.
    pub fn join(self, other: Self, join_type: JoinType) -> Result<Self, JoinFuncError> {
        if join_type == JoinType::Base {
            return Err(JoinFuncError::NonBaseJoin);
        }

        Ok(Self {
            lhs: Some(Box::new(self)),
            rhs: Some(Box::new(other)),
            join_type,
            base: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JoinType {
    Base,
    Sum,
    Product,
    Composition,
}

#[derive(thiserror::Error, Debug)]
pub enum JoinFuncError {
    #[error("Attempted two join two functions")]
    NonBaseJoin,
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{E, PI};

    use super::{Join, JoinType};
    use crate::expr::base_func::BaseFunc;

    #[test]
    fn test_sum() {
        let f = Join::new(BaseFunc::Ln);
        let g = Join::new(BaseFunc::Sin);

        let h = f.join(g, JoinType::Sum).unwrap();

        assert_eq!(h.eval(0.5), -0.21372164195574228);
    }

    #[test]
    fn test_product() {
        let f = Join::new(BaseFunc::Ln);
        let g = Join::new(BaseFunc::Sin);

        let h = f.join(g, JoinType::Product).unwrap();

        assert_eq!(h.eval(0.5), -0.3323124603719365);
    }

    #[test]
    fn test_composition() {
        let f = Join::new(BaseFunc::Ln);
        let g = Join::new(BaseFunc::Sin);

        let h = f.join(g, JoinType::Composition).unwrap();

        assert_eq!(h.eval(PI / 4.0), -0.34657359027997275);
    }

    #[test]
    fn test_difference() {
        let f = Join::new(BaseFunc::Arctan);
        let g = Join::new(BaseFunc::Constant(-1.0));
        // h(x) = -arctanx
        let h = f.join(g, JoinType::Product).unwrap();

        // i(x) = e^x
        let i = Join::new(BaseFunc::Exp(E));

        // j(x) = -arctanx * e^x
        let j = i.join(h, JoinType::Product).unwrap();

        assert_eq!(j.eval(PI), -29.218069233435283);
    }

    #[test]
    fn test_reciprocal() {
        let f = Join::new(BaseFunc::Arctan);
        let g = Join::new(BaseFunc::Power(-1.0));
        // h(x) = 1 / arctanx = 1 / (tan^-1 x) = tanx ????
        let h = g.join(f, JoinType::Composition).unwrap();

        // i(x) = e^x
        let i = Join::new(BaseFunc::Exp(E));

        // j(x) = e^x + 1 / arctanx
        let j = i.join(h, JoinType::Sum).unwrap();

        assert_eq!(j.eval(5.0), 149.14127869014925);
    }

    #[test]
    fn prevent_illegal_join() {
        let f = Join::new(BaseFunc::Arccos);
        let g = Join::new(BaseFunc::Ln);
        assert!(f.join(g, JoinType::Base).is_err());
    }
}
