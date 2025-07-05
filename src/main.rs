pub mod expr;
pub mod lex;
pub mod parser;

use lex::SubExpr;
use expr::syntax::*;

fn main() {
    let expr_str = "1 + 25 * 3 / 2sin(x) * e";
    
    let tokens = lex::tokenize(expr_str.into());
    
    println!("-----------");
    let result = parser::parse(tokens);
    match result {
        Ok(expr) => { dbg!(expr); },
        Err(err) => { println!("{err}"); }
    }
    
}
