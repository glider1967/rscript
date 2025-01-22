use core::fmt;

use crate::{tokenize::Span, types::Type};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Position {
    pub line: u32,
    pub col: u32,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ExprSpan {
    pub start: Position,
    pub end: Position,
}

impl fmt::Display for ExprSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{} - {}:{}",
            self.start.line + 1,
            self.start.col,
            self.end.line + 1,
            self.end.col - 1
        )
    }
}

impl ExprSpan {
    pub fn from_two_span(start: &Span, end: &Span) -> ExprSpan {
        ExprSpan {
            start: Position {
                line: start.line,
                col: start.start,
            },
            end: Position {
                line: end.line,
                col: end.end,
            },
        }
    }

    pub fn from_two_exprspan(a: &ExprSpan, b: &ExprSpan) -> ExprSpan {
        ExprSpan {
            start: a.start,
            end: b.end,
        }
    }

    pub fn from_span_and_exprspan(a: &Span, b: &ExprSpan) -> ExprSpan {
        ExprSpan {
            start: Position {
                line: a.line,
                col: a.start,
            },
            end: b.end,
        }
    }

    pub fn from(span: Span) -> ExprSpan {
        ExprSpan {
            start: Position {
                line: span.line,
                col: span.start,
            },
            end: Position {
                line: span.line,
                col: span.end,
            },
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SpannedExpr {
    pub expr: Expr,
    pub span: ExprSpan,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    Variable(String),
    Program(Vec<SpannedExpr>, Box<SpannedExpr>),
    BinOp(String, Box<SpannedExpr>, Box<SpannedExpr>),
    UnaryOp(String, Box<SpannedExpr>),
    If(Box<SpannedExpr>, Box<SpannedExpr>, Box<SpannedExpr>),
    Assign(String, Option<Type>, Box<SpannedExpr>),
    Reassign(String, Box<SpannedExpr>),
    EnumDef(String, Vec<ConstructorDef>),
    Match(Box<SpannedExpr>, Vec<(Pattern, SpannedExpr)>),
    Lambda(String, Option<Type>, Box<SpannedExpr>),
    App(Box<SpannedExpr>, Box<SpannedExpr>),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ConstructorDef {
    pub name: String,
    pub args: Vec<Type>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Pattern {
    pub name: String,
    pub vars: Vec<String>,
}

impl SpannedExpr {
    pub fn new(expr: Expr, span: ExprSpan) -> Self {
        SpannedExpr { expr, span }
    }

    pub fn int(num: i64, span: Span) -> Self {
        SpannedExpr::new(Expr::Int(num), ExprSpan::from(span))
    }

    pub fn boolean(b: bool, span: Span) -> Self {
        SpannedExpr::new(Expr::Bool(b), ExprSpan::from(span))
    }

    pub fn variable(name: String, span: Span) -> Self {
        SpannedExpr::new(Expr::Variable(name), ExprSpan::from(span))
    }

    pub fn binary_op(op: String, left: SpannedExpr, right: SpannedExpr) -> Self {
        let span = ExprSpan::from_two_exprspan(&left.span, &right.span);
        SpannedExpr::new(Expr::BinOp(op, Box::new(left), Box::new(right)), span)
    }

    pub fn unary_op(op: String, expr: SpannedExpr, span: Span) -> Self {
        let expr_span = expr.span.clone();
        SpannedExpr::new(
            Expr::UnaryOp(op, Box::new(expr)),
            ExprSpan::from_span_and_exprspan(&span, &expr_span),
        )
    }

    pub fn if_expr(
        cond: SpannedExpr,
        then: SpannedExpr,
        else_: SpannedExpr,
        span: ExprSpan,
    ) -> Self {
        SpannedExpr::new(
            Expr::If(Box::new(cond), Box::new(then), Box::new(else_)),
            span,
        )
    }

    pub fn assign(name: String, ty: Option<Type>, expr: SpannedExpr, span: ExprSpan) -> Self {
        SpannedExpr::new(Expr::Assign(name, ty, Box::new(expr)), span)
    }

    pub fn reassign(name: String, expr: SpannedExpr, span: ExprSpan) -> Self {
        SpannedExpr::new(Expr::Reassign(name, Box::new(expr)), span)
    }

    pub fn enum_def(name: String, defs: Vec<ConstructorDef>, span: ExprSpan) -> Self {
        SpannedExpr::new(Expr::EnumDef(name, defs), span)
    }

    pub fn match_expr(
        expr: SpannedExpr,
        arms: Vec<(Pattern, SpannedExpr)>,
        span: ExprSpan,
    ) -> Self {
        SpannedExpr::new(Expr::Match(Box::new(expr), arms), span)
    }

    pub fn lambda(param: String, ty: Option<Type>, body: SpannedExpr, span: ExprSpan) -> Self {
        SpannedExpr::new(Expr::Lambda(param, ty, Box::new(body)), span)
    }

    pub fn app(func: SpannedExpr, arg: SpannedExpr, span: ExprSpan) -> Self {
        SpannedExpr::new(Expr::App(Box::new(func), Box::new(arg)), span)
    }

    pub fn program(exprs: Vec<SpannedExpr>, last: SpannedExpr, span: ExprSpan) -> Self {
        SpannedExpr::new(Expr::Program(exprs, Box::new(last)), span)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Int(v) => write!(f, "{v}"),
            Expr::Bool(v) => write!(f, "{v}"),
            Expr::Variable(name) => write!(f, "{name}"),
            Expr::Program(v, ret) => write!(
                f,
                "{} {}",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                ret.to_string()
            ),
            Expr::BinOp(op, exp1, exp2) => write!(f, "({exp1} {op} {exp2})"),
            Expr::UnaryOp(op, expr) => write!(f, "{op}({expr})"),
            Expr::If(cond, exp1, exp2) => {
                write!(f, "if ({cond}) {{ {exp1} }} else {{ {exp2} }}")
            }
            Expr::Assign(ident, ty, expr) => {
                let tt = if ty.is_some() {
                    ty.as_ref().unwrap().to_string()
                } else {
                    "?".to_string()
                };
                write!(f, "let {ident}: {tt} = {expr};")
            }
            Expr::Reassign(ident, expr) => {
                write!(f, "mut {ident} = {expr};")
            }
            Expr::EnumDef(name, defs) => {
                write!(
                    f,
                    "enum {name}: {{ {} }};",
                    defs.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Expr::Match(expr, arms) => {
                write!(
                    f,
                    "match ({expr}) {{ {} }}",
                    arms.iter()
                        .map(|(p, e)| format!("{p} => {{ {e} }}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Expr::Lambda(var, ty, expr) => {
                let tt = if ty.is_some() {
                    ty.as_ref().unwrap().to_string()
                } else {
                    "?".to_string()
                };
                write!(f, "lambda ({var}: {tt}) {{{expr} }}")
            }
            Expr::App(fun, var) => {
                write!(f, "{fun}({var})")
            }
        }
    }
}

impl fmt::Display for ConstructorDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            self.name,
            self.args
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.vars.join(", "))
    }
}

impl fmt::Display for SpannedExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expr = &self.expr;
        write!(f, "{expr}")
    }
}
