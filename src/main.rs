pub mod expr;
pub mod lex;
pub mod parser;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let expr_str = "2^30(x)^2 / 3^4sin(x) + 2a";
    let result = parser::parse_str(expr_str.into());

    match result {
        Ok(expr) => {
            let simple = expr.simplified();
            dbg!(simple);
        },
        Err(err) => { println!("{err}"); }
    }
    let end = Instant::now();
    let dur = end.duration_since(start);
    println!("It took {:?} to complete", dur);

}
