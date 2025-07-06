mod derivative;
pub mod expr;
mod simplify;

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

    pub fn func<T: Into<String>>(name: T, arg: Expr) -> Expr {
        Expr::Func(name.into(), Box::new(arg))
    }

    pub fn sin(arg: Expr) -> Expr {
        func("sin", arg)
    }

    pub fn cos(arg: Expr) -> Expr {
        func("cos", arg)
    }

    pub fn tan(arg: Expr) -> Expr {
        func("tan", arg)
    }

    pub const X: Expr = Expr::Var('x');

    impl Expr {
        pub fn plus(self, that: Expr) -> Self {
            Expr::Add(Box::new(self), Box::new(that))
        }

        pub fn minus(self, that: Expr) -> Self {
            Expr::Sub(Box::new(self), Box::new(that))
        }

        pub fn times(self, that: Expr) -> Self {
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
            Derivative { expr: self }
        }
    }
}

pub use expr::*;
