use crate::expr::Expr;
use crate::lex::SubExpr;

fn function_of_s(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    // windows of size 4 because we're looking for [F(name), OpenPar, Arg(single S), ClosePar]
    it
        .windows(4)
        .enumerate()
        .find_map(| (index, slice) | {
            if let [SubExpr::F(name), SubExpr::OpenPar, SubExpr::S(arg), SubExpr::ClosePar] = slice {
                Some((index, SubExpr::S(Expr::Func(name.clone(), Box::new(arg.clone())))))      
            } else {
                None
            }
        })
}

fn s_in_parenthesis(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    // windows of size 3 because we're looking for [OpenPar, S(_), ClosePar]
    it
        .windows(3)
        .enumerate()
        .find_map(| (index, slice) | {
            if let [SubExpr::OpenPar, s@SubExpr::S(_), SubExpr::ClosePar] = slice {
                Some((index, s.clone()))
            } else {
                None
            }
        })
}

fn s_and_s_exponentiation(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    // windows of size 3 because we're looking for [S(_), ^, S(_)]
    it
        .windows(3)
        .enumerate()
        .find_map(| (index, slice) | {
            if let [SubExpr::S(base), SubExpr::Pow, SubExpr::S(exp)] = slice {
                let expr = Expr::Pow(
                    Box::new(base.clone()),
                    Box::new(exp.clone())
                );
                Some((index, SubExpr::S(expr)))
            } else {
                None
            }
        })
}

fn s_and_s_multiplication(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    // windows of size 3 because we're looking for [S(_), *, S(_)]
    it
        .windows(3)
        .enumerate()
        .find_map(| (index, slice) | {
            if let [SubExpr::S(lhs), SubExpr::Mul, SubExpr::S(rhs)] = slice {
                let expr = Expr::Mul(
                    Box::new(lhs.clone()),
                    Box::new(rhs.clone())
                );
                Some((index, SubExpr::S(expr)))
            } else {
                None
            }
        })
}

fn s_and_s_division(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    // windows of size 3 because we're looking for [S(_), /, S(_)]
    it
        .windows(3)
        .enumerate()
        .find_map(| (index, slice) | {
            if let [SubExpr::S(lhs), SubExpr::Div, SubExpr::S(rhs)] = slice {
                let expr = Expr::Div(
                    Box::new(lhs.clone()),
                    Box::new(rhs.clone())
                );
                Some((index, SubExpr::S(expr)))
            } else {
                None
            }
        })
}

fn s_and_s_addition(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    // windows of size 3 because we're looking for [S(_), +, S(_)]
    it
        .windows(3)
        .enumerate()
        .find_map(| (index, slice) | {
            if let [SubExpr::S(lhs), SubExpr::Plus, SubExpr::S(rhs)] = slice {
                let expr = Expr::Add(
                    Box::new(lhs.clone()),
                    Box::new(rhs.clone())
                );
                Some((index, SubExpr::S(expr)))
            } else {
                None
            }
        })
}

fn s_and_s_subtraction(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    // windows of size 3 because we're looking for [S(_), -, S(_)]
    it
        .windows(3)
        .enumerate()
        .find_map(| (index, slice) | {
            if let [SubExpr::S(lhs), SubExpr::Minus, SubExpr::S(rhs)] = slice {
                let expr = Expr::Sub(
                    Box::new(lhs.clone()),
                    Box::new(rhs.clone())
                );
                Some((index, SubExpr::S(expr)))
            } else {
                None
            }
        })
}

pub fn parse(tokens: Vec<SubExpr>) -> Result<Expr, String> {
    let mut remaining_tokens = tokens;
    while !matches!(&remaining_tokens[..], &[SubExpr::S(_)]) {
        if let Some((f_index, f)) = function_of_s(&remaining_tokens) {
            _ = remaining_tokens.splice(f_index..f_index+4, [f]);
        } else if let Some((s_index, s)) = s_in_parenthesis(&remaining_tokens) {
            _ = remaining_tokens.splice(s_index..s_index+3, [s]);
        } else if let Some((exp_index, exp)) = s_and_s_exponentiation(&remaining_tokens) {
            _ = remaining_tokens.splice(exp_index..exp_index+3, [exp]);
        } else if let Some((mul_index, mul)) = s_and_s_multiplication(&remaining_tokens) {
            _ = remaining_tokens.splice(mul_index..mul_index +3, [mul]);
        } else if let Some((div_index, div)) = s_and_s_division(&remaining_tokens) {
            _ = remaining_tokens.splice(div_index..div_index +3, [div]);
        } else if let Some((addition_index, addition)) = s_and_s_addition(&remaining_tokens) {
            _ = remaining_tokens.splice(addition_index..addition_index +3, [addition]);
        } else if let Some((sub_index, sub)) = s_and_s_subtraction(&remaining_tokens) {
            _ = remaining_tokens.splice(sub_index..sub_index +3, [sub]);
        }
        else {
            return Err(format!("Unrecognizable pattern: {:?}", &remaining_tokens));
        }
    }
    
    dbg!(&remaining_tokens);
    if let [SubExpr::S(e)] = &remaining_tokens[..] {
        Ok(e.clone())
    } else {
        Err("Something went wrong".into())
    }
    
}