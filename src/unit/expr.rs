use tokeniser::Tokeniser;

#[allow(dead_code)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, i32),
    Equal(Box<Expr>, Box<Expr>),
    Func(String, Box<Expr>),
    Val(String)
}

use std::ops::{Add, Sub, Mul, Div};

impl Add for Expr {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Expr::Add(Box::new(self), Box::new(rhs))
    }
}

impl Sub for Expr {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Expr::Sub(Box::new(self), Box::new(rhs))
    }
}

impl Mul for Expr {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Expr::Mul(Box::new(self), Box::new(rhs))
    }
}

impl Div for Expr {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Expr::Div(Box::new(self), Box::new(rhs))
    }
}

#[allow(dead_code)]
impl Expr {
    fn powi(self, rhs: i32) -> Self {
        Expr::Pow(Box::new(self), rhs)
    }
    fn eq(self, rhs: Self) -> Self {
        Expr::Equal(Box::new(self), Box::new(rhs))
    }
    fn func(self, f: String) -> Self {
        Expr::Func(f, Box::new(self))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
    LBracket,
    RBracket,
    Powi(i32),
    // Func(String),
}

fn power(s: &str) -> Option<i32> {
    if !s.chars().any(|c| c.is_digit(10) || c == '-') {
        s.chars().map(::super_to_num).collect::<String>().parse().ok()
    } else {
        None
    }
}

impl Operator {
    fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            "(" => Operator::LBracket,
            ")" => Operator::RBracket,
            "^" => Operator::Pow,
            "="|"=="|":=" => Operator::Eq,
            _ => if let Some(pow) = power(s) {
                Operator::Powi(pow)
            // } else if s.chars().all(|s| s.is_alphabetic()) {
                // Operator::Func(s.to_owned())
            } else {
                return None;
            }
        })
    }
    fn precedence(&self) -> u8 {
        use self::Operator::*;
        match *self {
            Eq => 0,
            Add|Sub => 1,
            Mul => 2,
            Div => 3,
            Pow => 4,
            Powi(_) => 5,
            RBracket => 10,
            LBracket => 0,
        }
    }
}

#[derive(Debug)]
enum Arg {
    Op(Operator),
    Val(String),
}

impl Expr {
    pub fn new(tokens: Tokeniser) -> Option<Self> {
        let mut operator_stack = Vec::<Operator>::new();
        let mut postfix = Vec::<Arg>::new();

        for token in tokens {
            if let Some(op) = Operator::from_str(&token) {
                while let Some(other_op) = operator_stack.pop() {
                    if op.precedence() > other_op.precedence() {
                        operator_stack.push(other_op);
                        break;
                    }
                    postfix.push(Arg::Op(other_op));
                }
                if op == Operator::RBracket {
                    let mut operator = operator_stack.pop()?;
                    while operator != Operator::LBracket {
                        postfix.push(Arg::Op(operator));
                        operator = operator_stack.pop()?;
                    }
                } else {
                    operator_stack.push(op);
                }
            } else { // it's an operand
                postfix.push(Arg::Val(token));
            }
        }

        postfix.extend(operator_stack.into_iter().map(Arg::Op));

        println!("{:?}", postfix);
        None
    }
    pub fn from_str(s: &str) -> Option<Self> {
        Self::new(Tokeniser::new(s))
    }
}
