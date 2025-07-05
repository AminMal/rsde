pub mod expr;
pub mod lex;
pub mod parser;

use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let expr_str = "2^10(x)^2 / (3^4)*x + 2x";
    let result = parser::parse_str(expr_str.into());

    match result {
        Ok(expr) => {
            let simple = expr.simplified();
            let vars = HashMap::from([('x', 1.0)]);
            let der = simple.derivative().with_respect_to('x').unwrap().simplified();
            println!("value at point x = 1 is: {}", simple.solve_for(&vars).unwrap());
            dbg!(&der);
            println!("derivative of expression at point x = 1 is {}", der.solve_for(&vars).unwrap());
            dbg!(simple);
        }
        Err(err) => {
            println!("{err}");
        }
    }
    let end = Instant::now();
    let dur = end.duration_since(start);
    println!("It took {:?} to complete", dur);
}
