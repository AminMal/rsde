use crate::expr::Expr;

pub struct Derivative<'a> {
    pub expr: &'a Expr
}

impl <'a> Derivative<'a> {
    pub fn with_respect_to(&self, var: char) -> Result<Expr, String> {
        if self.expr.is_effectively_constant() {
            Ok(Expr::Const(0))
        } else {
            match self.expr {
                Expr::Var(v) =>
                    if *v == var {
                        Ok(Expr::Const(1))
                    } else {
                        Err("derivative of other variables with respect to another variable has not been implemented yet".into())
                    }
                Expr::Neg(e) => {
                    let e_der = Self {expr: e.as_ref() };
                    Ok(
                        Expr::Neg(
                            Box::new(e_der.with_respect_to(var)?)
                        )
                    )
                },
                Expr::Add(lhs, rhs) => {
                    let lhs_der = Self { expr: lhs.as_ref()};
                    let rhs_der = Self { expr: rhs.as_ref()};
                    Ok(
                        Expr::Add(
                            Box::new(lhs_der.with_respect_to(var)?),
                            Box::new(rhs_der.with_respect_to(var)?)
                        )
                    )
                },
                Expr::Sub(lhs, rhs) => {
                    let lhs_der = Self { expr: lhs.as_ref()};
                    let rhs_der = Self { expr: rhs.as_ref()};
                    Ok(
                        Expr::Sub(
                            Box::new(lhs_der.with_respect_to(var)?),
                            Box::new(rhs_der.with_respect_to(var)?)
                        )
                    )
                },
                Expr::Mul(lhs, rhs) => {
                    let lhs_der = Self { expr: lhs.as_ref()};
                    let rhs_der = Self { expr: rhs.as_ref()};
                    // lhs'rhs + rhs'lhs
                    let result = Expr::Add(
                        Box::new(
                            Expr::Mul(
                                Box::new(lhs_der.with_respect_to(var)?),
                                rhs.clone()
                            )
                        ),
                        Box::new(
                            Expr::Mul(
                                Box::new(rhs_der.with_respect_to(var)?),
                                lhs.clone()
                            )
                        )
                    );
                    Ok(result)
                },
                Expr::Div(lhs, rhs) => {
                    let lhs_der = Self { expr: lhs.as_ref()};
                    let rhs_der = Self { expr: rhs.as_ref()};
                    // (lhs'rhs - rhs'lhs)/(rhs^2)
                    let dividend = Expr::Sub(
                        Box::new(
                            Expr::Mul(
                                Box::new(lhs_der.with_respect_to(var)?),
                                rhs.clone()
                            )
                        ),
                        Box::new(
                            Expr::Mul(
                                Box::new(rhs_der.with_respect_to(var)?),
                                lhs.clone()
                            )
                        )
                    );
                    let divisor = Expr::Pow(
                        rhs.clone(),
                        Box::new(Expr::Const(2))
                    );
                    Ok(Expr::Div(Box::new(dividend), Box::new(divisor)))
                },
                Expr::Pow(lhs, rhs) if matches!(lhs.as_ref(), &Expr::Var(_)) => {
                    if let (&Expr::Var(v), &Expr::Const(c)) = (lhs.as_ref(), rhs.as_ref()) {
                        if v == var && c > 0 {
                            Ok(
                                Expr::Mul(
                                    rhs.clone(),
                                    Box::new(
                                        Expr::Pow(lhs.clone(), Box::new(Expr::Const(c - 1)))
                                    )
                                )
                            )
                        } else {
                            Err("Other forms of pow not implemented yet".into())
                        }
                    } else {
                        Err("Other forms of pow not implemented yet".into())
                    }
                }
                Expr::Pow(_, _) => {
                    todo!()
                },
                Expr::Func(_, _) => {
                    todo!()
                },
                _ => {
                    Err("not implemented yet!".into())
                }
            }
        }
    }
}