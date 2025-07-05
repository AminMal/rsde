use crate::expr::Expr;

pub fn simplify(expr: Expr) -> Expr {
    match expr {
        Expr::Const(_) | Expr::Var(_) | Expr::E => expr,
        Expr::Neg(inner) if *inner == Expr::Const(0) => Expr::Const(0),
        Expr::Neg(inner) if matches!(*inner, Expr::Neg(_)) => inner.neg(),
        Expr::Add(lhs, rhs) if matches!(*rhs, Expr::Neg(_)) => Expr::Sub(lhs, Box::new(rhs.neg())),
        Expr::Add(lhs, rhs) if matches!(*lhs, Expr::Neg(_)) => Expr::Sub(rhs, Box::new(lhs.neg())),
        Expr::Add(lhs, rhs) => {
            let lhs_simplified = simplify(*lhs);
            let rhs_simplified = simplify(*rhs);
            if lhs_simplified == Expr::Const(0) {
                rhs_simplified
            } else if rhs_simplified == Expr::Const(0) {
                lhs_simplified
            } else {
                Expr::Add(Box::new(lhs_simplified), Box::new(rhs_simplified))
            }
        }
        Expr::Mul(lhs, rhs) => {
            let lhs_simplified = simplify(*lhs);
            let rhs_simplified = simplify(*rhs);
            if lhs_simplified == Expr::Const(0) || rhs_simplified == Expr::Const(0) {
                Expr::Const(0)
            } else if lhs_simplified == Expr::Const(1) {
                rhs_simplified
            } else if rhs_simplified == Expr::Const(1) {
                lhs_simplified
            } else {
                Expr::Mul(Box::new(lhs_simplified), Box::new(rhs_simplified))
            }
        }
        Expr::Pow(base, exponent) => {
            let base_simplified = simplify(*base);
            let exponent_simplified = simplify(*exponent);

            if exponent_simplified == Expr::Const(0) {
                Expr::Const(1)
            } else if exponent_simplified == Expr::Const(1) {
                base_simplified
            } else if base_simplified == Expr::Const(0) {
                Expr::Const(0)
            } else if base_simplified == Expr::Const(1) {
                Expr::Const(1)
            } else {
                Expr::Pow(Box::new(base_simplified), Box::new(exponent_simplified))
            }
        }
        Expr::Div(dividend, divisor) => {
            let dividend_simplified = simplify(*dividend);
            let divisor_simplified = simplify(*divisor);
            if divisor_simplified == Expr::Const(1) {
                divisor_simplified
            } else if divisor_simplified == dividend_simplified {
                Expr::Const(1)
            } else {
                Expr::Div(Box::new(dividend_simplified), Box::new(divisor_simplified))
            }
        }
        Expr::Func(name, arg) => Expr::Func(name, Box::new(simplify(*arg))),
        other => other,
    }
}
