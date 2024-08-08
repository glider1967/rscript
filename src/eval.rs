use std::{cell::RefCell, rc::Rc};

use anyhow::{bail, Ok, Result};

use crate::{environment::Env, expression::Expr, internal_value::Value};

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
        match &ast {
            Expr::Int(v) => Ok(Value::Int(*v)),
            Expr::Bool(v) => Ok(Value::Bool(*v)),
            Expr::Ident(name) => self.env.borrow().get(name),
            Expr::Program(prog, ret) => {
                for expr in prog {
                    let _ = self.eval(&expr);
                }
                self.eval(&ret)
            }
            Expr::BinOp(op, exp1, exp2) => {
                let v1 = self.eval(&exp1)?;
                let v2 = self.eval(&exp2)?;
                match (v1, v2) {
                    (Value::Int(x), Value::Int(y)) => match op.as_str() {
                        "+" => Ok(Value::Int(x + y)),
                        "-" => Ok(Value::Int(x - y)),
                        "*" => Ok(Value::Int(x * y)),
                        "/" => Ok(Value::Int(x / y)),
                        "==" => Ok(Value::Bool(x == y)),
                        "!=" => Ok(Value::Bool(x != y)),
                        "<" => Ok(Value::Bool(x < y)),
                        ">" => Ok(Value::Bool(x > y)),
                        "<=" => Ok(Value::Bool(x <= y)),
                        ">=" => Ok(Value::Bool(x >= y)),
                        _ => bail!("invalid binary operation {}", op),
                    },
                    (Value::Bool(x), Value::Bool(y)) => match op.as_str() {
                        "&&" => Ok(Value::Bool(x && y)),
                        "||" => Ok(Value::Bool(x || y)),
                        _ => bail!("invalid binary operation {}", op),
                    },
                    _ => {
                        bail!("invalid binary operation {}", op)
                    }
                }
            }
            Expr::UnaryOp(op, exp1) => {
                let v1 = self.eval(&exp1)?;
                match v1 {
                    Value::Int(x) => match op.as_str() {
                        "-" => Ok(Value::Int(-x)),
                        _ => bail!("invalid binary operation {}", op),
                    },
                    Value::Bool(x) => match op.as_str() {
                        "!" => Ok(Value::Bool(!x)),
                        _ => bail!("invalid binary operation {}", op),
                    },
                    _ => {
                        bail!("invalid binary operation {}", op)
                    }
                }
            }
            Expr::If(cond, exp1, exp2) => {
                if let Value::Bool(b) = self.eval(&cond)? {
                    if b {
                        self.eval(&exp1)
                    } else {
                        self.eval(&exp2)
                    }
                } else {
                    bail!("if expression: non-bool condition!");
                }
            }
            Expr::Assign(name, _, expr) => {
                let val = self.eval(&expr)?;
                self.env.borrow_mut().set(name, val.clone());
                Ok(val)
            }
            Expr::Lambda(var, _, expr) => {
                let new_env = Env::with_outer(Rc::clone(&self.env));
                Ok(Value::Lambda(var.clone(), expr.clone(), new_env))
            }
            Expr::App(fun, var) => {
                if let Value::Lambda(arg, expr, env) = self.eval(&fun)? {
                    let inner_eval = Eval::with_env(env);
                    inner_eval.env.borrow_mut().set(&arg, self.eval(&var)?);
                    inner_eval.eval(&expr)
                } else {
                    bail!("eval error: application to non-lambda!")
                }
            }
        }
    }
}
