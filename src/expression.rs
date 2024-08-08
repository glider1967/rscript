use core::fmt;

use crate::types::Type;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    Variable(String),
    Program(Vec<Expr>, Box<Expr>),
    BinOp(String, Box<Expr>, Box<Expr>),
    UnaryOp(String, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Assign(String, Option<Type>, Box<Expr>),
    Lambda(String, Option<Type>, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn int(num: i64) -> Self {
        Expr::Int(num)
    }

    pub fn boolean(b: bool) -> Self {
        Expr::Bool(b)
    }

    pub fn variable(name: String) -> Self {
        Expr::Variable(name)
    }

    pub fn assign(name: String, ty: Option<Type>, expr: Expr) -> Self {
        Expr::Assign(name, ty, Box::new(expr))
    }

    pub fn binop(name: String, exp1: Expr, exp2: Expr) -> Self {
        Expr::BinOp(name, Box::new(exp1), Box::new(exp2))
    }

    pub fn unaryop(name: String, expr: Expr) -> Self {
        Expr::UnaryOp(name, Box::new(expr))
    }

    pub fn app(fun: Expr, arg: Expr) -> Self {
        Expr::App(Box::new(fun), Box::new(arg))
    }

    pub fn if_expr(cond: Expr, expr: Expr, elseexp: Expr) -> Self {
        Expr::If(Box::new(cond), Box::new(expr), Box::new(elseexp))
    }

    pub fn lambda(name: String, argty: Option<Type>, expr: Expr) -> Self {
        Expr::Lambda(name, argty, Box::new(expr))
    }

    pub fn program(prog: Vec<Expr>, ret: Expr) -> Self {
        Expr::Program(prog, Box::new(ret))
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Int(v) => write!(f, "Int({})", v),
            Expr::Bool(v) => write!(f, "{}", v),
            Expr::Variable(name) => write!(f, "{}", name),
            Expr::Program(v, ret) => write!(
                f,
                "{} {}",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                ret.to_string()
            ),
            Expr::BinOp(op, exp1, exp2) => write!(f, "({} {op} {})", exp1, exp2),
            Expr::UnaryOp(op, expr) => write!(f, "{op}{}", expr),
            Expr::If(cond, exp1, exp2) => {
                write!(f, "if ({}) {{ {} }} else {{ {} }}", cond, exp1, exp2)
            }
            Expr::Assign(ident, ty, expr) => {
                let tt = if ty.is_some() {
                    ty.as_ref().unwrap().to_string()
                } else {
                    "?".to_string()
                };
                write!(f, "let {ident}: {} = {};", tt, expr)
            }
            Expr::Lambda(var, ty, expr) => {
                let tt = if ty.is_some() {
                    ty.as_ref().unwrap().to_string()
                } else {
                    "?".to_string()
                };
                write!(f, "lambda ({var}:{}) {{ {} }}", tt, expr)
            }
            Expr::App(fun, var) => {
                write!(f, "{}({})", fun, var)
            }
        }
    }
}
