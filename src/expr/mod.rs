pub mod expr;
mod simplify;
mod derivative;

use derivative::Derivative;

pub mod syntax {
    use crate::expr::derivative::Derivative;
    use crate::expr::simplify::simplify;
    use crate::expr::Expr;

    pub fn num(n: u32) -> Expr {
        Expr::Const(n)
    }

    pub const E: Expr = Expr::E;

    pub fn var(name: char) -> Expr {
        Expr::Var(name)
    }

    impl Expr {
        pub fn add(self, that: Expr) -> Self {
            Expr::Add(Box::new(self), Box::new(that))
        }

        pub fn sub(self, that: Expr) -> Self {
            Expr::Sub(Box::new(self), Box::new(that))
        }

        pub fn mul(self, that: Expr) -> Self {
            Expr::Mul(Box::new(self), Box::new(that))
        }

        pub fn div(self, that: Expr) -> Self {
            Expr::Div(Box::new(self), Box::new(that))
        }

        pub fn pow(self, that: Expr) -> Self {
            Expr::Pow(Box::new(self), Box::new(that))
        }

        pub fn neg(self) -> Self {
            match self {
                Expr::Neg(inner) => *inner,
                _ => Expr::Neg(Box::new(self)),
            }
        }

        pub fn simplified(self) -> Self {
            simplify(self)
        }
        
        pub fn derivative(&self) -> Derivative {
            Derivative{ expr: self }
        }
    }
}

pub use expr::*;
