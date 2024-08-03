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
    Assign(String, Box<Expr>),
    Lambda(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Expr {
    pub expr: InnerExpr,
    pub ty: Type,
}

impl Expr {
    pub fn int(num: i64) -> Self {
        Self {
            expr: InnerExpr::Int(num),
            ty: Type::Int,
        }
    }

    pub fn boolean(b: bool) -> Self {
        Self {
            expr: InnerExpr::Bool(b),
            ty: Type::Bool,
        }
    }

    pub fn ident(name: String, ty: Type) -> Self {
        Self {
            expr: InnerExpr::Ident(name),
            ty,
        }
    }

    pub fn assign(name: String, expr: Expr) -> Self {
        Self {
            expr: InnerExpr::Assign(name, Box::new(expr)),
            ty: Type::Unit,
        }
    }

    pub fn binop(name: String, exp1: Expr, exp2: Expr) -> Self {
        let ty = if exp1.ty == exp2.ty {
            exp1.ty.clone()
        } else {
            panic!("wtf")
        };

        Self {
            expr: InnerExpr::BinOp(name, Box::new(exp1), Box::new(exp2)),
            ty,
        }
    }

    pub fn unaryop(name: String, expr: Expr) -> Self {
        Self {
            expr: InnerExpr::UnaryOp(name, Box::new(expr.clone())),
            ty: expr.ty,
        }
    }

    pub fn app(fun: Expr, arg: Expr) -> Self {
        if let Type::Func(dom, cod) = &fun.ty {
            if arg.ty == **dom {
                Self {
                    expr: InnerExpr::App(Box::new(fun.clone()), Box::new(arg)),
                    ty: *cod.clone(),
                }
            } else {
                panic!("type");
            }
        } else {
            panic!("type")
        }
    }

    pub fn if_expr(cond: Expr, expr: Expr, elseexp: Expr) -> Self {
        Self {
            expr: InnerExpr::If(Box::new(cond), Box::new(expr.clone()), Box::new(elseexp)),
            ty: expr.ty,
        }
    }

    pub fn lambda(name: String, argty: Type, expr: Expr) -> Self {
        Self {
            expr: InnerExpr::Lambda(name, Box::new(expr.clone())),
            ty: Type::func(argty, expr.ty),
        }
    }

    pub fn program(prog: Vec<Expr>, ret: Expr) -> Self {
        Self {
            expr: InnerExpr::Program(prog, Box::new(ret.clone())),
            ty: ret.ty,
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
            InnerExpr::Assign(ident, expr) => {
                write!(f, "let {ident} = {};", expr.expr)
            }
            InnerExpr::Lambda(var, _) => {
                write!(f, "<lambda ({})>", var)
            }
            InnerExpr::App(fun, var) => {
                write!(f, "{}({})", fun.expr, var.expr)
            }
        }
    }
}
