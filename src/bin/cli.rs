use rsde::expr::Expr;
use rsde::out::*;
use rsde::parser;
use std::collections::{HashMap, VecDeque};
use std::env::args;

fn help() {
    println!("help!");
}

struct ApplicationArgs {
    expr: Option<Expr>,
    derivative: bool,
    derivative_over: Option<char>,
    simplify: bool,
    vars: HashMap<char, f64>,
    out: Box<dyn Out>,
}

impl ApplicationArgs {
    fn add_var(&mut self, name: char, value: f64) {
        self.vars.insert(name, value);
    }

    fn set_expr(&mut self, expr: Expr) {
        self.expr = Some(expr);
    }

    fn der(&mut self, value: bool) {
        self.derivative = value;
    }

    fn simplify(&mut self) {
        self.simplify = true;
    }

    fn over(&mut self, var: char) {
        self.derivative_over = Some(var);
    }

    fn out(&mut self, output: Box<dyn Out>) {
        self.out = output;
    }
}

fn parse_var(key_value_pair: String) -> Result<(char, f64), String> {
    if let [key_str, value_str] = key_value_pair.split("=").collect::<Vec<_>>()[..] {
        match (key_str.chars().next(), value_str.parse::<f64>()) {
            (Some(var_name), Ok(value)) => Ok((var_name, value)),
            _ => Err(format!("invalid argument pattern: [{}]", key_value_pair)),
        }
    } else {
        Err(format!("invalid argument pattern: [{}]", key_value_pair))
    }
}

fn parse_app_args() -> Result<ApplicationArgs, String> {
    let mut result = ApplicationArgs {
        expr: None,
        derivative: false,
        derivative_over: None,
        simplify: false,
        vars: HashMap::new(),
        out: Box::new(standard()),
    };
    let mut a = args().collect::<VecDeque<_>>();

    while !a.is_empty() {
        match a.pop_front().unwrap().as_str() {
            "--expr" | "-e" => match a.pop_front() {
                None => {
                    return Err("An expression has to be provided after arg --expr".into());
                }
                Some(value) => {
                    let exp = parser::parse_str(value)?;
                    result.set_expr(exp);
                }
            },
            "--at" => match a.pop_front() {
                Some(key_value_pair) => {
                    let (var_name, value) = parse_var(key_value_pair)?;
                    result.add_var(var_name, value);
                }
                None => {
                    return Err("No value provided after --at".into());
                }
            },
            "--derivative" | "-d" => {
                result.der(true);
            }
            "--with-respect-to" | "-r" => match a.pop_front() {
                Some(var_name_str) => match var_name_str.chars().next() {
                    Some(var_name) => {
                        result.over(var_name);
                    }
                    None => {
                        return Err("No variable specified after --with-respect-to".into());
                    }
                },
                _ => {
                    return Err("No variable specified after --with-respect-to".into());
                }
            },
            "--simplify" | "-s" => {
                result.simplify();
            }
            "--help" | "-h" => {
                help();
            }
            "--output" | "-o" => match a.pop_front().unwrap_or(String::default()).as_str() {
                "standard" => {
                    result.out(Box::new(standard()));
                }
                "latex" => {
                    result.out(Box::new(latex()));
                }
                "" => {
                    return Err("no output specified, run with --help".into());
                }
                other => {
                    return Err(format!("not a valid output {}", other));
                }
            },
            _ => {}
        }
    }

    Ok(result)
}

fn main() -> Result<(), String> {
    match parse_app_args()? {
        // derivative
        ApplicationArgs {
            expr: Some(e),
            derivative: true,
            derivative_over: Some(var),
            vars,
            out,
            ..
        } => {
            let der = e.derivative().with_respect_to(var)?.simplified();
            println!("derivative formula: {}", out.output(&der)?);
            if vars.contains_key(&var) {
                let value = der.solve_for(&vars)?;
                println!("derivative at specified point(s) is {}", value);
            }
            Ok(())
        }
        // simplify
        ApplicationArgs {
            expr: Some(e),
            derivative: false,
            simplify: true,
            out,
            ..
        } => {
            let simplified = e.simplified();
            println!("simplified expression:\n{}", out.output(&simplified)?);
            Ok(())
        }
        // solve
        ApplicationArgs {
            expr: Some(e),
            derivative: false,
            simplify: false,
            vars,
            ..
        } => {
            println!(
                "expression value at given point(s) is {}",
                e.solve_for(&vars)?
            );
            Ok(())
        }
        _ => Err("unrecognizable CLI args pattern".into()),
    }
}
