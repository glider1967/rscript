use std::{cell::RefCell, rc::Rc};

use anyhow::{bail, Ok, Result};

use crate::{environment::Env, expression::Expr};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Lambda(String, Box<Expr>, Env),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Bool(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Lambda(v, _, _) => format!("lambda ({v})"),
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

    pub fn eval_expr(&self, ast: &Expr) -> Result<Value> {
        match ast {
            Expr::Int(v) => Ok(Value::Int(*v)),
            Expr::Bool(v) => Ok(Value::Bool(*v)),
            Expr::Ident(name) => self.env.borrow().get(name.clone()),
            Expr::Program(prog, ret) => {
                for expr in prog {
                    let _ = self.eval_expr(expr);
                }
                self.eval_expr(ret)
            }
            Expr::BinPlus(exp1, exp2) => Ok(int_bin_op(
                Box::new(|x, y| x + y),
                self.eval_expr(exp1)?,
                self.eval_expr(exp2)?,
            )?),
            Expr::BinMinus(exp1, exp2) => Ok(int_bin_op(
                Box::new(|x, y| x - y),
                self.eval_expr(exp1)?,
                self.eval_expr(exp2)?,
            )?),
            Expr::BinMult(exp1, exp2) => Ok(int_bin_op(
                Box::new(|x, y| x * y),
                self.eval_expr(exp1)?,
                self.eval_expr(exp2)?,
            )?),
            Expr::BinDiv(exp1, exp2) => Ok(int_bin_op(
                Box::new(|x, y| x / y),
                self.eval_expr(exp1)?,
                self.eval_expr(exp2)?,
            )?),
            Expr::BinEq(exp1, exp2) => Ok(int_bin_op_bool(
                Box::new(|x, y| x == y),
                self.eval_expr(exp1)?,
                self.eval_expr(exp2)?,
            )?),
            Expr::BinNotEq(exp1, exp2) => Ok(int_bin_op_bool(
                Box::new(|x, y| x != y),
                self.eval_expr(exp1)?,
                self.eval_expr(exp2)?,
            )?),
            Expr::BinLT(exp1, exp2) => Ok(int_bin_op_bool(
                Box::new(|x, y| x < y),
                self.eval_expr(exp1)?,
                self.eval_expr(exp2)?,
            )?),
            Expr::BinGT(exp1, exp2) => Ok(int_bin_op_bool(
                Box::new(|x, y| x > y),
                self.eval_expr(exp1)?,
                self.eval_expr(exp2)?,
            )?),
            Expr::BinLE(exp1, exp2) => Ok(int_bin_op_bool(
                Box::new(|x, y| x <= y),
                self.eval_expr(exp1)?,
                self.eval_expr(exp2)?,
            )?),
            Expr::BinGE(exp1, exp2) => Ok(int_bin_op_bool(
                Box::new(|x, y| x >= y),
                self.eval_expr(exp1)?,
                self.eval_expr(exp2)?,
            )?),
            Expr::BinAnd(exp1, exp2) => Ok(bool_bin_op(
                Box::new(|x, y| x && y),
                self.eval_expr(exp1)?,
                self.eval_expr(exp2)?,
            )?),
            Expr::BinOr(exp1, exp2) => Ok(bool_bin_op(
                Box::new(|x, y| x || y),
                self.eval_expr(exp1)?,
                self.eval_expr(exp2)?,
            )?),
            Expr::UnaryMinus(exp1) => {
                Ok(int_unary_op(Box::new(|x: i64| -x), self.eval_expr(exp1)?)?)
            }
            Expr::UnaryNot(exp1) => Ok(bool_unary_op(
                Box::new(|x: bool| !x),
                self.eval_expr(exp1)?,
            )?),
            Expr::If(cond, exp1, exp2) => {
                if let Value::Bool(b) = self.eval_expr(cond)? {
                    if b {
                        self.eval_expr(exp1)
                    } else {
                        self.eval_expr(exp2)
                    }
                } else {
                    bail!("if expression: non-bool condition!");
                }
            }
            Expr::Assign(name, expr) => {
                let val = self.eval_expr(expr)?;
                self.env.borrow_mut().set(name.clone(), val.clone());
                Ok(val)
            }
            Expr::Lambda(var, expr) => {
                let new_env = Env::with_outer(Rc::clone(&self.env));
                Ok(Value::Lambda(var.clone(), expr.clone(), new_env))
            }
            Expr::App(fun, var) => {
                if let Value::Lambda(arg, expr, env) = self.eval_expr(fun)? {
                    let inner_eval = Eval::with_env(env);
                    inner_eval.env.borrow_mut().set(arg, self.eval_expr(var)?);
                    inner_eval.eval_expr(&expr)
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
