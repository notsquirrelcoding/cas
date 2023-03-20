use super::base_func::BaseFunc;

pub struct Join {
    lhs: Option<Box<Join>>,
    rhs: Option<Box<Join>>,
    join_type: JoinType,
    base: Option<BaseFunc>
}

impl Join {
    pub fn new(f: BaseFunc) -> Join {
        Join { lhs: None, rhs: None, join_type: JoinType::Base, base: Some(f) }
    }
}

pub enum JoinType {
    Base,
    Sum,
    Product,
    Composition
}