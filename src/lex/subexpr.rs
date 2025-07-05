use std::fmt::{Display, Formatter};
use crate::expr::Expr;

#[derive(Debug, PartialEq, Clone)]
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