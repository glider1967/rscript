use std::{cell::RefCell, rc::Rc};
use core::fmt;

use anyhow::{bail, Ok, Result};

use crate::{environment::Env, expression::Expr};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Lambda(String, Box<Expr>, Env),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Int(i) => write!(f, "{i}"),
            Value::Lambda(v, _, _) => write!(f, "lambda ({v})"),
        }
    }
}

pub struct Eval {
    env: Rc<RefCell<Env>>,
}

impl Eval {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Env::new())),
        }
    }

    pub fn with_env(env: Env) -> Self {
        Self {
            env: Rc::new(RefCell::new(env)),
        }
    }

    pub fn eval(&self, ast: &Expr) -> Result<Value> {
        match ast {
            Expr::Int(v) => Ok(Value::Int(*v)),
            Expr::Bool(v) => Ok(Value::Bool(*v)),
            Expr::Ident(name) => self.env.borrow().get(name.clone()),
            Expr::Program(prog, ret) => {
                for expr in prog {
                    let _ = self.eval(expr);
                }
                self.eval(ret)
            }
            Expr::BinPlus(exp1, exp2) => Ok(int_bin_op(
                Box::new(|x, y| x + y),
                self.eval(exp1)?,
                self.eval(exp2)?,
            )?),
            Expr::BinMinus(exp1, exp2) => Ok(int_bin_op(
                Box::new(|x, y| x - y),
                self.eval(exp1)?,
                self.eval(exp2)?,
            )?),
            Expr::BinMult(exp1, exp2) => Ok(int_bin_op(
                Box::new(|x, y| x * y),
                self.eval(exp1)?,
                self.eval(exp2)?,
            )?),
            Expr::BinDiv(exp1, exp2) => Ok(int_bin_op(
                Box::new(|x, y| x / y),
                self.eval(exp1)?,
                self.eval(exp2)?,
            )?),
            Expr::BinEq(exp1, exp2) => Ok(int_bin_op_bool(
                Box::new(|x, y| x == y),
                self.eval(exp1)?,
                self.eval(exp2)?,
            )?),
            Expr::BinNotEq(exp1, exp2) => Ok(int_bin_op_bool(
                Box::new(|x, y| x != y),
                self.eval(exp1)?,
                self.eval(exp2)?,
            )?),
            Expr::BinLT(exp1, exp2) => Ok(int_bin_op_bool(
                Box::new(|x, y| x < y),
                self.eval(exp1)?,
                self.eval(exp2)?,
            )?),
            Expr::BinGT(exp1, exp2) => Ok(int_bin_op_bool(
                Box::new(|x, y| x > y),
                self.eval(exp1)?,
                self.eval(exp2)?,
            )?),
            Expr::BinLE(exp1, exp2) => Ok(int_bin_op_bool(
                Box::new(|x, y| x <= y),
                self.eval(exp1)?,
                self.eval(exp2)?,
            )?),
            Expr::BinGE(exp1, exp2) => Ok(int_bin_op_bool(
                Box::new(|x, y| x >= y),
                self.eval(exp1)?,
                self.eval(exp2)?,
            )?),
            Expr::BinAnd(exp1, exp2) => Ok(bool_bin_op(
                Box::new(|x, y| x && y),
                self.eval(exp1)?,
                self.eval(exp2)?,
            )?),
            Expr::BinOr(exp1, exp2) => Ok(bool_bin_op(
                Box::new(|x, y| x || y),
                self.eval(exp1)?,
                self.eval(exp2)?,
            )?),
            Expr::UnaryMinus(exp1) => Ok(int_unary_op(Box::new(|x: i64| -x), self.eval(exp1)?)?),
            Expr::UnaryNot(exp1) => Ok(bool_unary_op(Box::new(|x: bool| !x), self.eval(exp1)?)?),
            Expr::If(cond, exp1, exp2) => {
                if let Value::Bool(b) = self.eval(cond)? {
                    if b {
                        self.eval(exp1)
                    } else {
                        self.eval(exp2)
                    }
                } else {
                    bail!("if expression: non-bool condition!");
                }
            }
            Expr::Assign(name, expr) => {
                let val = self.eval(expr)?;
                self.env.borrow_mut().set(name.clone(), val.clone());
                Ok(val)
            }
            Expr::Lambda(var, expr) => {
                let new_env = Env::with_outer(Rc::clone(&self.env));
                Ok(Value::Lambda(var.clone(), expr.clone(), new_env))
            }
            Expr::App(fun, var) => {
                if let Value::Lambda(arg, expr, env) = self.eval(fun)? {
                    let inner_eval = Eval::with_env(env);
                    inner_eval.env.borrow_mut().set(arg, self.eval(var)?);
                    inner_eval.eval(&expr)
                } else {
                    bail!("eval error: application to non-lambda!")
                }
            }
        }
    }
}

fn int_bin_op<F>(func: F, v1: Value, v2: Value) -> Result<Value>
where
    F: Fn(i64, i64) -> i64,
{
    match (v1, v2) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(func(a, b))),
        _ => bail!("int binop for non-integer!"),
    }
}

fn int_bin_op_bool<F>(func: F, v1: Value, v2: Value) -> Result<Value>
where
    F: Fn(i64, i64) -> bool,
{
    match (v1, v2) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(func(a, b))),
        _ => bail!("int binop for non-integer!"),
    }
}

fn bool_bin_op<F>(func: F, v1: Value, v2: Value) -> Result<Value>
where
    F: Fn(bool, bool) -> bool,
{
    match (v1, v2) {
        (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(func(a, b))),
        _ => bail!("bool binop for non-bool!"),
    }
}

fn int_unary_op<F>(func: F, v1: Value) -> Result<Value>
where
    F: Fn(i64) -> i64,
{
    match v1 {
        Value::Int(a) => Ok(Value::Int(func(a))),
        _ => bail!("int unary for non-integer!"),
    }
}

fn bool_unary_op<F>(func: F, v1: Value) -> Result<Value>
where
    F: Fn(bool) -> bool,
{
    match v1 {
        Value::Bool(a) => Ok(Value::Bool(func(a))),
        _ => bail!("bool unary for non-bool!"),
    }
}
