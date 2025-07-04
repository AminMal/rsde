use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub enum SubExpr {
    S(Expr),
    F(String),
    OpenPar,
    ClosePar,
    Plus,
    Minus,
    Mul,
    Div,
    Pow
}