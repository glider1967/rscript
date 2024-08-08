use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Func(Box<Type>, Box<Type>),
}

impl Type {
    pub fn func(t1: Type, t2: Type) -> Self {
        Type::Func(Box::new(t1), Box::new(t2))
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Bool => write!(f, "bool"),
            Type::Int => write!(f, "int"),
            Type::Func(t1, t2) => write!(f, "({t1} -> {t2})"),
        }
    }
}

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::{bail, Ok, Result};

use crate::expression::Expr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeEnv {
    env: HashMap<String, Type>,
    outer: Option<Rc<RefCell<TypeEnv>>>,
}

impl TypeEnv {
    fn new() -> Self {
        Self {
            env: HashMap::new(),
            outer: None,
        }
    }

    fn with_outer(outer: Rc<RefCell<TypeEnv>>) -> Self {
        Self {
            env: HashMap::new(),
            outer: Some(outer),
        }
    }

    fn get(&self, name: String) -> Result<Type> {
        if let Some(val) = self.env.get(&name) {
            Ok(val.clone())
        } else if let Some(outer) = &self.outer {
            outer.borrow().get(name)
        } else {
            bail!("type: undefined variable {name}");
        }
    }

    fn set(&mut self, name: String, val: Type) {
        self.env.insert(name, val);
    }
}

pub struct TypeInfer {
    env: Rc<RefCell<TypeEnv>>,
}

impl TypeInfer {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(TypeEnv::new())),
        }
    }

    fn from(env: TypeEnv) -> Self {
        Self {
            env: Rc::new(RefCell::new(env)),
        }
    }

    pub fn infer_type(&mut self, ast: &Expr) -> Result<Type> {
        match &ast {
            Expr::Int(_) => Ok(Type::Int),
            Expr::Bool(_) => Ok(Type::Bool),
            Expr::Ident(name) => {
                let actual_type = self.env.borrow().get(name.clone())?;
                Ok(actual_type)
            }
            Expr::Program(v, ret) => {
                for expr in v {
                    let _ = self.infer_type(expr)?;
                }
                let ret_type = self.infer_type(&ret)?;
                Ok(ret_type)
            }
            Expr::BinOp(op, exp1, exp2) => match op.as_str() {
                "+" | "-" | "*" | "/" => {
                    let t1 = self.infer_type(&exp1)?;
                    let t2 = self.infer_type(&exp2)?;
                    if t1 != t2 {
                        bail!(
                            "invalid type: binop:{} left:{} right:{}",
                            op.as_str(),
                            t1,
                            t2
                        )
                    }
                    Ok(Type::Int)
                }
                "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                    let t1 = self.infer_type(&exp1)?;
                    let t2 = self.infer_type(&exp2)?;
                    if t1 != t2 {
                        bail!(
                            "invalid type: binop:{} left:{} right:{}",
                            op.as_str(),
                            t1,
                            t2
                        )
                    }
                    Ok(Type::Int)
                }
                "&&" | "||" => {
                    let t1 = self.infer_type(&exp1)?;
                    let t2 = self.infer_type(&exp2)?;
                    if t1 != t2 {
                        bail!(
                            "invalid type: binop:{} left:{} right:{}",
                            op.as_str(),
                            t1,
                            t2
                        )
                    }
                    Ok(Type::Bool)
                }
                _ => bail!("invalid operator: {}", op),
            },
            Expr::UnaryOp(op, expr) => match op.as_str() {
                "-" => {
                    let t1 = self.infer_type(&expr)?;
                    if t1 != Type::Int {
                        bail!("invalid type: op:{} arg:{}", op.as_str(), t1)
                    }
                    Ok(Type::Int)
                }
                "!" => {
                    let t1 = self.infer_type(&expr)?;
                    if t1 != Type::Bool {
                        bail!("invalid type: op:{} arg:{}", op.as_str(), t1)
                    }
                    Ok(Type::Bool)
                }
                _ => bail!("invalid operator: {}", op),
            },
            Expr::If(cond, exp1, exp2) => {
                let t0 = self.infer_type(&cond)?;
                let t1 = self.infer_type(&exp1)?;
                let t2 = self.infer_type(&exp2)?;
                if t0 != Type::Bool {
                    bail!("invalid type: condition of if expr is not bool")
                }
                if t1 != t2 {
                    bail!("invalid type: if expr right:{} else:{}", t1, t2)
                }
                Ok(t1)
            }
            Expr::Assign(ident, ty, expr) => {
                let actual = self.infer_type(&expr)?;
                if ty != &actual {
                    bail!("invalid type of assign: expected:{} actual:{}", ty, actual)
                }
                self.env.borrow_mut().set(ident.clone(), ty.clone());
                Ok(actual)
            }
            Expr::Lambda(var, ty, expr) => {
                let mut new_type_infer = Self::from(TypeEnv::with_outer(Rc::clone(&self.env)));
                new_type_infer.env.borrow_mut().set(var.clone(), ty.clone());
                let ret_type = new_type_infer.infer_type(&expr)?;
                Ok(Type::func(ty.clone(), ret_type))
            }
            Expr::App(fun, var) => {
                let fun_type = self.infer_type(&fun)?;
                let var_type = self.infer_type(&var)?;
                if let Type::Func(dom, cod) = fun_type {
                    if *dom != var_type {
                        bail!(
                            "invalid type of application: expected:{} actual:{}",
                            dom,
                            var_type
                        )
                    }
                    Ok(*cod)
                } else {
                    bail!("invalid func type")
                }
            }
        }
    }
}
