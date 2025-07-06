use crate::expr::Expr;
use crate::lex::subexpr::SubExpr;
use std::collections::VecDeque;

// TODO, completely unsafe module here, refactor later

// TODO, fix this
fn unsafe_char_to_u32(c: char) -> u32 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => {
            panic!("not a digit {c}");
        }
    }
}

fn fold_while<T, Z, P, F>(l: &mut VecDeque<T>, p: P, zero: Z, f: F) -> Z
where
    P: Fn(&T) -> bool,
    F: Fn((Z, &T)) -> Z,
{
    let mut result: Z = zero;
    let len_matching_condition = l.iter().take_while(|&x| p(x)).count();
    for _ in 0..len_matching_condition {
        let elem = l.pop_front().unwrap();
        result = f((result, &elem));
    }
    result
}

fn trimmed_chars(s: String) -> VecDeque<char> {
    s.replace(" ", "").chars().collect()
}

pub fn tokenize(s: String) -> Result<Vec<SubExpr>, String> {
    let mut chars = trimmed_chars(s);
    let mut sub_expressions: Vec<SubExpr> = Vec::new();

    while !chars.is_empty() {
        let next = chars.pop_front().ok_or("expected next char")?;
        match next {
            '^' => sub_expressions.push(SubExpr::Pow),
            '*' => sub_expressions.push(SubExpr::Mul),
            '+' => sub_expressions.push(SubExpr::Plus),
            '-' => sub_expressions.push(SubExpr::Minus),
            '(' => sub_expressions.push(SubExpr::OpenPar),
            ')' => sub_expressions.push(SubExpr::ClosePar),
            '/' => sub_expressions.push(SubExpr::Div),
            // nums
            c if c.is_numeric() => {
                let num = fold_while(
                    &mut chars,
                    |x| x.is_numeric(),
                    unsafe_char_to_u32(c),
                    |(n, &elem)| n * 10 + unsafe_char_to_u32(elem),
                );
                sub_expressions.push(SubExpr::S(Expr::Const(num)));
                // if next is a variable, a function, or parenthesis, then that implicitly means multiplication:
                match chars.get(0) {
                    Some(&x) => {
                        if x.is_alphabetic() || x == '(' {
                            chars.push_front('*');
                        }
                    }
                    None => (),
                }
            }
            a if a.is_alphabetic() => {
                let next = chars.get(0);
                let next_is_alphabetic = next.is_some_and(|x| x.is_alphabetic());
                let next_is_paren = next.is_some_and(|&x| x == '(');
                // vars and e
                if !next_is_alphabetic && !next_is_paren {
                    let sub_expr = match a {
                        'e' => SubExpr::S(Expr::E),
                        _ => SubExpr::S(Expr::Var(a)),
                    };
                    sub_expressions.push(sub_expr)
                } else {
                    // func symbols
                    let name = fold_while(
                        &mut chars,
                        |x| x.is_alphabetic(),
                        String::from(a),
                        |(s, ch)| format!("{}{}", s, ch),
                    );
                    sub_expressions.push(SubExpr::F(name))
                }
            }
            other => {
                println!("[WARN] Skipping unrecognized character [{other}]");
            }
        }
    }

    Ok(sub_expressions)
}
