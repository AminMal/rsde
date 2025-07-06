use crate::expr::Expr;

pub fn simplify(expr: Expr) -> Expr {
    match expr {
        Expr::Const(_) | Expr::Var(_) | Expr::E => expr,
        Expr::Neg(inner) if *inner == Expr::Const(0) => Expr::Const(0),
        Expr::Neg(inner) if matches!(*inner, Expr::Neg(_)) => inner.neg(),
        Expr::Add(lhs, rhs) if matches!(*rhs, Expr::Neg(_)) => Expr::Sub(lhs, Box::new(rhs.neg())),
        Expr::Add(lhs, rhs) if matches!(*lhs, Expr::Neg(_)) => Expr::Sub(rhs, Box::new(lhs.neg())),
        Expr::Sub(lhs, rhs) if *lhs == *rhs => Expr::Const(0),
        Expr::Sub(lhs, rhs) if matches!(*rhs, Expr::Neg(_)) => Expr::Add(lhs, Box::new(rhs.neg())),
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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::expr::{syntax::*, Expr};

    #[test]
    fn test_add_0_is_self() -> Result<(), String> {
        let expr = simplify(num(12).plus(num(0)));
        if let Expr::Const(12) = expr {
            Ok(())
        } else {
            Err(format!("{:?} did not equal Const(12)", expr))
        }
    }

    #[test]
    fn test_mul_0_is_0() -> Result<(), String> {
        let expr = simplify(num(12).times(num(0)));
        if let Expr::Const(0) = expr {
            Ok(())
        } else {
            Err(format!("{:?} did not equal Const(12)", expr))
        }
    }

    #[test]
    fn test_mul_1_is_self() -> Result<(), String> {
        let expr = simplify(num(12).times(num(1)));
        if let Expr::Const(12) = expr {
            Ok(())
        } else {
            Err(format!("{:?} did not equal Const(12)", expr))
        }
    }

    #[test]
    fn neg_of_neg_is_self() -> Result<(), String> {
        let expr = simplify(num(65).times(num(12).minus(num(6))));
        let double_negged = Expr::Neg(Box::new(Expr::Neg(Box::new(expr.clone()))));
        if expr == simplify(double_negged) {
            Ok(())
        } else {
            Err("double negation did not equal self!".into())
        }
    }

    #[test]
    fn neg_of_0_is_0() -> Result<(), String> {
        let expr = simplify(Expr::Neg(Box::new(num(0))));
        if let Expr::Const(0) = expr {
            Ok(())
        } else {
            Err("Neg of 0 did not equal Const(0)".into())
        }
    }

    // 3 + (-2) => 3 - 2
    #[test]
    fn add_of_negated_rhs_is_sub() -> Result<(), String> {
        let expr = simplify(num(3).plus(num(2).neg()));
        let expected = Expr::Sub(Box::new(num(3)), Box::new(num(2)));
        if expr == expected {
            Ok(())
        } else {
            Err("add of negated arg did not equal sub".into())
        }
    }

    // -(3) + 2 => 2 - 3
    #[test]
    fn add_of_negated_lhs_is_sub() -> Result<(), String> {
        let expr = simplify(num(3).neg().plus(num(2)));
        let expected = Expr::Sub(Box::new(num(2)), Box::new(num(3)));
        if expr == expected {
            Ok(())
        } else {
            Err("add of negated arg did not equal sub".into())
        }
    }

    // 2 - -(3) => 2 + 3
    #[test]
    fn sub_of_negated_rhs_is_add() -> Result<(), String> {
        let expr = simplify(num(2).minus(num(3).neg()));
        let expected = num(2).plus(num(3));
        if expr == expected {
            Ok(())
        } else {
            Err("subtraction of negated arg did not equal sub".into())
        }
    }

    #[test]
    fn expr_minus_self_is_0() -> Result<(), String> {
        let common = num(110).plus(num(2).times(X).plus(num(77)));
        let expr = simplify(common.clone().minus(common));
        let expected = num(0);

        if expr == expected {
            Ok(())
        } else {
            Err("expression minus itself did not equal 0".into())
        }
    }

    #[test]
    fn to_pow_1_is_self() -> Result<(), String> {
        // (x + 2)^1 => x + 2
        let expr = simplify(X.plus(num(2)).pow(num(1)));
        let expected = X.plus(num(2));
        if expr == expected {
            Ok(())
        } else {
            Err("Expr to the power of 1 did not equal the expression itself".into())
        }
    }

    #[test]
    fn to_pow_0_is_1() -> Result<(), String> {
        // (x + 2)^1 => x + 2
        let expr = simplify(X.plus(num(2)).pow(num(0)));
        let expected = num(1);
        if expr == expected {
            Ok(())
        } else {
            Err("Expr to the power of 0 did not equal Const(1)".into())
        }
    }

    #[test]
    fn _1_to_pow_anything_is_1() -> Result<(), String> {
        // 1^(x+e) => 1
        let expr = simplify(num(1).pow(X.plus(E)));
        let expected = num(1);
        if expr == expected {
            Ok(())
        } else {
            Err("1 to the power of expression did not equal 1".into())
        }
    }

    #[test]
    fn _0_to_pow_anything_is_0() -> Result<(), String> {
        // 0^(x+e) => 0
        let expr = simplify(num(0).pow(X.plus(E)));
        let expected = num(0);
        if expr == expected {
            Ok(())
        } else {
            Err("0 to the power of some expression did not equal 0".into())
        }
    }

    #[test]
    fn anything_divided_by_itself_is_1() -> Result<(), String> {
        // (x+e)*(109+110)/(x+e)*(109+110) => 1
        let common_expr = (X.plus(E)).times(num(109).plus(num(110)));
        let expr = simplify(common_expr.clone().div(common_expr));
        let expected = num(1);
        if expr == expected {
            Ok(())
        } else {
            Err("expression divided by itself did not equal 1".into())
        }
    }

    #[test]
    fn func_arg_should_be_simplified() -> Result<(), String> {
        // fnc(109-109+10) => fnc(10)
        let expr = simplify(func("fnc", num(109).minus(num(109)).plus(num(10))));
        let expected = func("fnc", num(10));

        if expr == expected {
            Ok(())
        } else {
            dbg!(expr);
            Err("function arguments not simplified".into())
        }
    }
}
