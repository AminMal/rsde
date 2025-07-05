#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Const(u64),
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
}
