use crate::expr::Expr;

pub trait Out {
    fn output(&self, e: &Expr) -> Result<String, String>;
}

pub fn standard() -> impl Out {
    StandardOut {}
}

pub fn latex() -> impl Out {
    LatexOut {}
}

struct StandardOut {}

impl Out for StandardOut {
    fn output(&self, e: &Expr) -> Result<String, String> {
        match e {
            Expr::Var(c) => Ok(c.to_string()),
            Expr::Const(num) => Ok(num.to_string()),
            Expr::E => Ok("e".into()),
            Expr::Add(lhs, rhs) => Ok(format!("({} + {})", self.output(lhs)?, self.output(rhs)?)),
            Expr::Mul(lhs, rhs) if matches!(**rhs, Expr::Var(_)) => {
                Ok(format!("{}{}", self.output(lhs)?, self.output(rhs)?))
            }
            Expr::Mul(lhs, rhs) if matches!(**lhs, Expr::Var(_)) => {
                Ok(format!("{}{}", self.output(rhs)?, self.output(lhs)?))
            }
            Expr::Mul(lhs, rhs) => Ok(format!("({} * {})", self.output(lhs)?, self.output(rhs)?)),
            Expr::Sub(lhs, rhs) => Ok(format!("({} - {})", self.output(lhs)?, self.output(rhs)?)),
            Expr::Div(lhs, rhs) => Ok(format!("({} / {})", self.output(lhs)?, self.output(rhs)?)),
            Expr::Pow(base, exponent) => Ok(format!(
                "({}^{})",
                self.output(base)?,
                self.output(exponent)?
            )),
            Expr::Neg(expr) => Ok(format!("-{}", self.output(expr)?)),
            Expr::Func(name, expr) => Ok(format!("{}({})", name, self.output(expr)?)),
        }
    }
}

pub struct LatexOut {}

impl LatexOut {
    fn var_appears_next(e: &Expr) -> bool {
        match e {
            Expr::Var(_) | Expr::E => true,
            Expr::Pow(lhs, _) => Self::var_appears_next(lhs.as_ref()),
            _ => false,
        }
    }

    fn add_or_sub(e: &Expr) -> bool {
        match e {
            Expr::Add(_, _) | Expr::Sub(_, _) => true,
            _ => false,
        }
    }
}

impl Out for LatexOut {
    fn output(&self, e: &Expr) -> Result<String, String> {
        match e {
            Expr::Var(name) => Ok(name.to_string()),
            Expr::Const(num) => Ok(format!("{{{num}}}")),
            Expr::E => Ok("e".into()),
            Expr::Add(lhs, rhs) => Ok(format!("({} + {})", self.output(lhs)?, self.output(rhs)?)),
            Expr::Sub(lhs, rhs) => Ok(format!("({} - {})", self.output(lhs)?, self.output(rhs)?)),
            Expr::Mul(lhs, rhs) if matches!(**rhs, Expr::Func(_, _)) => {
                Ok(format!("{}{}", self.output(lhs)?, self.output(rhs)?))
            }
            Expr::Mul(lhs, rhs) if matches!(**lhs, Expr::Func(_, _)) => {
                Ok(format!("{}{}", self.output(rhs)?, self.output(lhs)?))
            }
            Expr::Mul(lhs, rhs) if Self::var_appears_next(rhs) => {
                Ok(format!("{}{}", self.output(lhs)?, self.output(rhs)?))
            }
            Expr::Mul(lhs, rhs) if Self::var_appears_next(lhs) => {
                Ok(format!("{}{}", self.output(rhs)?, self.output(lhs)?))
            }
            Expr::Mul(lhs, rhs)
                if !Self::add_or_sub(lhs.as_ref()) && !Self::add_or_sub(rhs.as_ref()) =>
            {
                Ok(format!("{} * {}", self.output(lhs)?, self.output(rhs)?))
            }
            Expr::Mul(lhs, rhs) => Ok(format!("({} * {})", self.output(lhs)?, self.output(rhs)?)),
            Expr::Pow(base, exponent) => Ok(format!(
                "{} ^ {}",
                self.output(base)?,
                self.output(exponent)?
            )),
            Expr::Div(dividend, divisor) => Ok(format!(
                "\\frac{{{}}}{{{}}}",
                self.output(dividend)?,
                self.output(divisor)?
            )),
            Expr::Neg(inner) => Ok(format!("-{}", self.output(inner)?)),
            Expr::Func(name, arg) => Ok(format!("{}({})", name, self.output(arg)?)),
        }
    }
}
