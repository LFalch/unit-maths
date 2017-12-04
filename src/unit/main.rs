extern crate unit_maths;

use std::io::stdin;
use std::collections::HashMap;

use unit_maths::*;

mod tokeniser;
mod infix;
mod expr;

#[derive(Debug)]
enum Eval {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Pow(String, String),
    Func(String, String),
}

#[derive(Debug)]
enum Command {
    Define(String, Value<f64>),
    Eval(Eval),
    Inspect(String),
    Assign(String, Eval),
}

// TODO split better
fn line_to_command(s: &str, si: &UnitSystem<f64>) -> Option<Command> {
    let mut words = s.split_whitespace().filter(|s| !s.is_empty());
    let first_arg = words.next()?.to_owned();

    Some(match words.next()? {
        "=" | ":=" => {
            let s = words.collect::<Vec<_>>().join(" ");
            let cmd = line_to_command(&s, si);
            if let Some(Command::Eval(eval)) = cmd {
                Command::Assign(first_arg, eval)
            } else {
                Command::Define(first_arg, si.val_s(&s)?)
            }
        }
        "?" => Command::Inspect(first_arg),
        "+" => Command::Eval(Eval::Add(first_arg, words.next()?.to_owned())),
        "-" => Command::Eval(Eval::Sub(first_arg, words.next()?.to_owned())),
        "*" => Command::Eval(Eval::Mul(first_arg, words.next()?.to_owned())),
        "/" => Command::Eval(Eval::Div(first_arg, words.next()?.to_owned())),
        "^" => Command::Eval(Eval::Pow(first_arg, words.next()?.to_owned())),
        snd => if first_arg.chars().all(<char>::is_alphabetic) {
            Command::Eval(Eval::Func(first_arg, snd.to_owned()))
        } else {
            return None;
        }
    })
}

fn evaluate(eval: Eval, vars: &HashMap<String, Value<f64>>) -> Option<Value<f64>> {
    let get_or_eval = |n: String| {
        vars.get(&n).cloned().or_else(|| {
            n.parse().ok().and_then(|v| Some(Value(v, Unit::new(NUL))))
        })
    };

    Some(match eval {
        Eval::Add(a, b) => get_or_eval(a)? + get_or_eval(b)?,
        Eval::Sub(a, b) => get_or_eval(a)? - get_or_eval(b)?,
        Eval::Mul(a, b) => get_or_eval(a)? * get_or_eval(b)?,
        Eval::Div(a, b) => get_or_eval(a)? / get_or_eval(b)?,
        Eval::Pow(a, b) => {
            let n: i16 = b.parse().ok()?;
            let Value(v, u) = get_or_eval(a)?;

            Value(v.powi(n as i32), u * n)
        }
        Eval::Func(a, b) => func(&a, *vars.get(&b)?),
    })
}

fn func(f: &str, val: Value<f64>) -> Value<f64> {
    match f {
        "p" => {
            assert_eq!(val.1.dimension, CONCENTRATION);
            Value(-(val.0 * val.1.factor / 1e3).log10(), Unit::new(NUL))
        }
        _ => panic!("No such function `{}'", f),
    }
}

fn main() {
    let si = UnitSystem::<f64>::si();
    let mut vars = HashMap::new();

    let mut s = String::new();
    'cmd_loop: loop {
        {
            stdin().read_line(&mut s).unwrap();
            let mut last_c = s.chars().last().unwrap();
            while last_c == '\n' || last_c == '\r' {
                s.pop();
                if let Some(c) = s.chars().last() {
                    last_c = c;
                } else {
                    continue 'cmd_loop;
                }
            }
        }

        let _expr = expr::Expr::from_str(&s);
        println!("Infix: {:?}", infix::infix_to_postfix(&s));

        if let Some(c) = line_to_command(&s, &si) {
            match c {
                Command::Define(name, val) => {
                    vars.insert(name, val);
                }
                Command::Eval(eval) => if let Some(val) = evaluate(eval, &vars) {
                    println!("= {}", si.display(&val));
                } else {
                    println!("No such variable");
                },
                Command::Assign(name, eval) => if let Some(val) = evaluate(eval, &vars) {
                    println!("= {}", si.display(&val));
                    vars.insert(name, val);
                } else {
                    println!("No such variable");
                },
                Command::Inspect(name) => if let Some(val) = vars.get(&name) {
                    println!("= {} ({:#})", si.display(&val), val.1.dimension);
                } else {
                    println!("No such variable");
                },
            }
        } else {
            if s == "stop" {
                break;
            }
            println!("Bad query");
        }
        s.clear();
    }
}
