use std::{cell::RefCell, rc::Rc};

use anyhow::{bail, Ok, Result};

use crate::{
    environment::Env,
    expression::{ConstructorDef, Expr, Pattern, SpannedExpr},
    internal_value::Value,
};

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

    pub fn eval(&self, ast: &SpannedExpr) -> Result<Value> {
        let expr = &ast.expr;
        match expr {
            Expr::Int(v) => Ok(Value::Int(*v)),
            Expr::Bool(v) => Ok(Value::Bool(*v)),
            Expr::Str(s) => Ok(Value::Str(s.clone())),
            Expr::Variable(name) => self.env.borrow().get(name),
            Expr::Program(prog, ret) => {
                for expr in prog {
                    self.eval(&expr)?;
                }
                self.eval(&ret)
            }
            Expr::BinOp(op, exp1, exp2) => {
                let v1 = self.eval(&exp1)?;
                let v2 = self.eval(&exp2)?;
                match (v1, v2) {
                    (Value::Str(x), Value::Str(y)) => match op.as_str() {
                        "++" => Ok(Value::Str(x + &y)),
                        _ => bail!("invalid binary operation {}", op),
                    },
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
                self.env.borrow_mut().set_new(name, val.clone())?;
                Ok(val)
            }
            Expr::Reassign(name, expr) => {
                let val = self.eval(&expr)?;
                self.env.borrow_mut().set_dup(name, val.clone())?;
                Ok(val)
            }
            Expr::EnumDef(_, defs) => {
                for ConstructorDef { name, args: _ } in defs {
                    self.env
                        .borrow_mut()
                        .set_new(name, Value::Constructor(name.to_owned(), vec![]))?;
                }
                Ok(Value::Unit)
            }
            Expr::Match(expr, arms) => {
                let val = self.eval(&expr)?;
                for (pattern, arm) in arms {
                    let new_env = Env::with_outer(Rc::clone(&self.env));
                    let inner_eval = Eval::with_env(new_env);
                    if inner_eval.match_pattern(&val, &pattern)? {
                        return inner_eval.eval(arm);
                    }
                }
                bail!("pattern doesn't match any pattern")
            }
            Expr::Lambda(var, _, expr) => {
                let new_env = Env::with_outer(Rc::clone(&self.env));
                Ok(Value::Lambda(var.clone(), expr.clone(), new_env))
            }
            Expr::App(fun, var) => {
                if let Value::Lambda(arg, expr, env) = self.eval(&fun)? {
                    let inner_eval = Eval::with_env(env);
                    inner_eval
                        .env
                        .borrow_mut()
                        .set_new(&arg, self.eval(&var)?)?;
                    inner_eval.eval(&expr)
                } else if let Value::Constructor(name, vals) = self.eval(&fun)? {
                    let mut vals = vals;
                    vals.push(self.eval(var)?);
                    Ok(Value::Constructor(name, vals))
                } else {
                    bail!("eval error: application to non-lambda or non-constructor!")
                }
            }
        }
    }

    pub fn match_pattern(&self, val: &Value, pattern: &Pattern) -> Result<bool> {
        if let Value::Constructor(constr_name, args) = val {
            if *constr_name != pattern.name {
                return Ok(false);
            }
            if pattern.vars.len() != args.len() {
                bail!("number of args does't match");
            }
            for i in 0..args.len() {
                self.env
                    .borrow_mut()
                    .set_new(&pattern.vars[i], args[i].clone())?;
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
