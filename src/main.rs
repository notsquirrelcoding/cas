mod expr;
use expr::{
    join::{Join, JoinType},
};

use crate::expr::base_func::BaseFunc;

fn main() {
    let f = Join::new(BaseFunc::Ln);
    let g = Join::new(BaseFunc::Power(-1.0));
    let h = g.join(f, JoinType::Composition).unwrap();

    println!("{}", h.eval(0.5));

}
