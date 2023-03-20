use super::base_func::BaseFunc;

pub struct Join {
    first: Box<Join>,
    second: Option<Box<Join>>,
    join_type: JoinType
}

impl Join {
    pub fn new(f: BaseFunc) -> Self {
    }
}

pub enum JoinType {
    Sum,
    Product,
    Composition
}