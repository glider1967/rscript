use core::fmt;

use crate::types::Type;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum InnerExpr {
    Int(i64),
    Bool(bool),
    Ident(String),
    Program(Vec<Expr>, Box<Expr>),
    BinOp(String, Box<Expr>, Box<Expr>),
    UnaryOp(String, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Assign(String, Type, Box<Expr>),
    Lambda(String, Type, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Expr {
    pub expr: InnerExpr,
}

impl Expr {
    pub fn int(num: i64) -> Self {
        Self {
            expr: InnerExpr::Int(num),
        }
    }

    pub fn boolean(b: bool) -> Self {
        Self {
            expr: InnerExpr::Bool(b),
        }
    }

    pub fn ident(name: String) -> Self {
        Self {
            expr: InnerExpr::Ident(name),
        }
    }

    pub fn assign(name: String, ty: Type, expr: Expr) -> Self {
        Self {
            expr: InnerExpr::Assign(name, ty, Box::new(expr)),
        }
    }

    pub fn binop(name: String, exp1: Expr, exp2: Expr) -> Self {
        Self {
            expr: InnerExpr::BinOp(name, Box::new(exp1), Box::new(exp2)),
        }
    }

    pub fn unaryop(name: String, expr: Expr) -> Self {
        Self {
            expr: InnerExpr::UnaryOp(name, Box::new(expr.clone())),
        }
    }

    pub fn app(fun: Expr, arg: Expr) -> Self {
        Self {
            expr: InnerExpr::App(Box::new(fun.clone()), Box::new(arg)),
        }
    }

    pub fn if_expr(cond: Expr, expr: Expr, elseexp: Expr) -> Self {
        Self {
            expr: InnerExpr::If(Box::new(cond), Box::new(expr.clone()), Box::new(elseexp)),
        }
    }

    pub fn lambda(name: String, argty: Type, expr: Expr) -> Self {
        Self {
            expr: InnerExpr::Lambda(name, argty, Box::new(expr.clone())),
        }
    }

    pub fn program(prog: Vec<Expr>, ret: Expr) -> Self {
        Self {
            expr: InnerExpr::Program(prog, Box::new(ret.clone())),
        }
    }
}

impl fmt::Display for InnerExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InnerExpr::Int(v) => write!(f, "Int({})", v),
            InnerExpr::Bool(v) => write!(f, "{}", v),
            InnerExpr::Ident(name) => write!(f, "{}", name),
            InnerExpr::Program(v, ret) => write!(
                f,
                "{} {}",
                v.iter()
                    .map(|x| x.expr.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                ret.expr.to_string()
            ),
            InnerExpr::BinOp(op, exp1, exp2) => write!(f, "({} {op} {})", exp1.expr, exp2.expr),
            InnerExpr::UnaryOp(op, expr) => write!(f, "{op}{}", expr.expr),
            InnerExpr::If(cond, exp1, exp2) => {
                write!(
                    f,
                    "if ({}) {{ {} }} else {{ {} }}",
                    cond.expr, exp1.expr, exp2.expr
                )
            }
            InnerExpr::Assign(ident, ty, expr) => {
                write!(f, "let {ident}: {ty} = {};", expr.expr)
            }
            InnerExpr::Lambda(var, ty, expr) => {
                write!(f, "lambda ({var}:{ty}) {{ {} }}", expr.expr)
            }
            InnerExpr::App(fun, var) => {
                write!(f, "{}({})", fun.expr, var.expr)
            }
        }
    }
}
