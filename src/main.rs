pub mod expr;
pub mod lex;

use lex::SubExpr;
use expr::syntax::*;

fn main() {
    let expr_str = "1 + 25 * 3 / 2sin(x) * e";
    
    let tokens = lex::tokenize(expr_str.into());
    
    assert_eq!(&tokens, &vec![
        SubExpr::S(num(1)),
        SubExpr::Plus,
        SubExpr::S(num(25)),
        SubExpr::Mul,
        SubExpr::S(num(3)),
        SubExpr::Div,
        SubExpr::S(num(2)),
        SubExpr::Mul,
        SubExpr::F("sin".into()),
        SubExpr::OpenPar,
        SubExpr::S(var('x')),
        SubExpr::ClosePar,
        SubExpr::Mul,
        SubExpr::S(E)
    ]);
    
    dbg!(tokens);
}
