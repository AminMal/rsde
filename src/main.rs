pub mod expr;

use crate::expr::syntax::*;

fn main() {
    let mulex = num(2).mul(num(3));
    let ex = num(1).add(num(3)).mul(E).sub(mulex.clone());
    let is_effectively_const = ex.is_effectively_constant();
    
    dbg!(ex);
    dbg!(mulex);
    println!("{is_effectively_const}")
}
