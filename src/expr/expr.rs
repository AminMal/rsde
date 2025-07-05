use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Const(u32),
    E,
    Var(char),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Func(String, Box<Expr>),
}

impl Expr {
    pub fn is_effectively_constant(&self) -> bool {
        match self {
            Expr::Const(_) => true,
            Expr::E => true,
            Expr::Add(lhs, rhs) => lhs.is_effectively_constant() && rhs.is_effectively_constant(),
            Expr::Sub(lhs, rhs) => lhs.is_effectively_constant() && rhs.is_effectively_constant(),
            Expr::Mul(lhs, rhs) => lhs.is_effectively_constant() && rhs.is_effectively_constant(),
            Expr::Div(dividend, divisor) => {
                dividend.is_effectively_constant() && divisor.is_effectively_constant()
            }
            Expr::Pow(base, exponent) => {
                base.is_effectively_constant() && exponent.is_effectively_constant()
            }
            Expr::Neg(e) => e.is_effectively_constant(),
            _ => false,
        }
    }
    
    pub fn solve_for(&self, vars: &HashMap<char, f64>) -> Result<f64, String> {
        match self {
            Expr::Const(n) => Ok(f64::from(*n)),
            Expr::Var(name) => vars.get(name).map(|&x| x).ok_or(format!("could not find variable [{}]", name)),
            Expr::E => Ok(std::f64::consts::E),
            Expr::Add(lhs, rhs) => Ok(lhs.solve_for(&vars)? + rhs.solve_for(&vars)?),
            Expr::Sub(lhs, rhs) => Ok(lhs.solve_for(&vars)? - rhs.solve_for(&vars)?),
            Expr::Mul(lhs, rhs) => Ok(lhs.solve_for(&vars)? * rhs.solve_for(&vars)?),
            Expr::Div(lhs, rhs) => Ok(lhs.solve_for(&vars)? / rhs.solve_for(&vars)?),
            Expr::Pow(lhs, rhs) => Ok(lhs.solve_for(&vars)?.powf(rhs.solve_for(&vars)?)),
            Expr::Neg(e) => Ok(-(e.solve_for(&vars)?)),
            Expr::Func(name, arg) => {
                match name.as_str() {
                    "sin" => Ok(arg.solve_for(&vars)?.sin()),
                    "cos" => Ok(arg.solve_for(&vars)?.cos()),
                    "tan" => Ok(arg.solve_for(&vars)?.tan()),
                    "cot" => Ok(1.0 / arg.solve_for(&vars)?.tan()),
                    other => Err(format!("Unrecognized function [{}({:?})]", name, arg))
                }
            }
        }
    }
}
