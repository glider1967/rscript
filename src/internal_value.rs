use core::fmt;

use crate::{environment::Env, expression::SpannedExpr};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Str(String),
    Lambda(String, Box<SpannedExpr>, Env),
    Constructor(String, Vec<Value>),
    Unit,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Int(i) => write!(f, "{i}"),
            Value::Str(s) => write!(f, "\"{s}\""),
            Value::Lambda(v, expr, _) => write!(f, "lambda ({v}) {{{expr} }}"),
            Value::Constructor(name, vals) => write!(
                f,
                "{name}({})",
                vals.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Unit => write!(f, "()"),
        }
    }
}
