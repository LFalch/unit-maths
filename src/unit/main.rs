extern crate unit_maths;

use std::io::stdin;
use std::collections::HashMap;

use unit_maths::*;

#[derive(Debug)]
enum Eval {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Pow(String, String),
    Func(String, String)
}

#[derive(Debug)]
enum Command {
    Define(String, Value<f64>),
    Eval(Eval),
    Assign(String, Eval)
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
    Some(match eval {
        Eval::Add(a, b) => *vars.get(&a)? + *vars.get(&b)?,
        Eval::Sub(a, b) => *vars.get(&a)? - *vars.get(&b)?,
        Eval::Mul(a, b) => *vars.get(&a)? * *vars.get(&b)?,
        Eval::Div(a, b) => *vars.get(&a)? / *vars.get(&b)?,
        Eval::Pow(_, _) => panic!("No powey wowy yet"),
        Eval::Func(a, b) => func(&a, *vars.get(&b)?),
    })
}

fn func(f: &str, val: Value<f64>) -> Value<f64> {
    match f {
        "p" => {
            assert_eq!(val.1.dimension, CONCENTRATION);
            Value(-(val.0 * val.1.factor/1e3).log10(), Unit::new(NUL))
        }
        _ => panic!("No such function `{}'", f)
    }
}

fn main() {
    let si = UnitSystem::<f64>::si();
    let mut vars = HashMap::new();

    let mut s = String::new();
    loop {
        {
            stdin().read_line(&mut s).unwrap();
            let mut last_c = s.chars().last().unwrap();
            while last_c == '\n' || last_c == '\r' {
                s.pop();
                last_c = s.chars().last().unwrap();
            }
        }

        if let Some(c) = line_to_command(&s, &si) {
            match c {
                Command::Define(name, val) => {
                    vars.insert(name, val);
                }
                Command::Eval(eval) => {
                    if let Some(val) = evaluate(eval, &vars) {
                        println!("= {}", si.display(&val));
                    } else {
                        println!("No such variable");
                    }
                }
                Command::Assign(name, eval) => {
                    if let Some(val) = evaluate(eval, &vars) {
                        println!("= {}", si.display(&val));
                        vars.insert(name, val);
                    } else {
                        println!("No such variable");
                    }
                }
            }
        } else {
            if s == "stop" {
                break
            }
            println!("Bad query");
        }
        s.clear();
    }
}
