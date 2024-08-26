use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Func(Box<Type>, Box<Type>),
    TypeVar(u64, Rc<RefCell<Option<Type>>>),
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
            Type::TypeVar(id, _) => write!(f, "t{id}"),
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
    next_typevar_id: u64,
}

impl TypeInfer {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(TypeEnv::new())),
            next_typevar_id: 0,
        }
    }

    fn from(env: TypeEnv) -> Self {
        Self {
            env: Rc::new(RefCell::new(env)),
            next_typevar_id: 0,
        }
    }

    fn new_typevar(&mut self) -> Type {
        let ret = Type::TypeVar(self.next_typevar_id, Rc::new(RefCell::new(None)));
        self.next_typevar_id += 1;
        ret
    }

    pub fn infer_type(&mut self, ast: &Expr) -> Result<Type> {
        match &ast {
            Expr::Int(_) => Ok(Type::Int),
            Expr::Bool(_) => Ok(Type::Bool),
            Expr::Variable(name) => {
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
                    Self::unify(&t1, &t2)?;
                    Ok(Type::Int)
                }
                "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                    let t1 = self.infer_type(&exp1)?;
                    let t2 = self.infer_type(&exp2)?;
                    Self::unify(&t1, &t2)?;
                    Ok(Type::Int)
                }
                "&&" | "||" => {
                    let t1 = self.infer_type(&exp1)?;
                    let t2 = self.infer_type(&exp2)?;
                    Self::unify(&t1, &t2)?;
                    Ok(Type::Bool)
                }
                _ => bail!("invalid operator: {}", op),
            },
            Expr::UnaryOp(op, expr) => match op.as_str() {
                "-" => {
                    let t1 = self.infer_type(&expr)?;
                    Self::unify(&t1, &Type::Int)?;
                    Ok(Type::Int)
                }
                "!" => {
                    let t1 = self.infer_type(&expr)?;
                    Self::unify(&t1, &Type::Bool)?;
                    Ok(Type::Bool)
                }
                _ => bail!("invalid operator: {}", op),
            },
            Expr::If(cond, exp1, exp2) => {
                let t0 = self.infer_type(&cond)?;
                let t1 = self.infer_type(&exp1)?;
                let t2 = self.infer_type(&exp2)?;
                Self::unify(&t0, &Type::Bool)?;
                Self::unify(&t1, &t2)?;
                Ok(t1)
            }
            Expr::Assign(ident, ty, expr) => {
                let nty = self.new_typevar();
                self.env.borrow_mut().set(ident.clone(), nty);
                let actual = self.infer_type(&expr)?;

                if let Some(expected) = ty {
                    Self::unify(expected, &actual)?;
                }
                self.env.borrow_mut().set(ident.clone(), actual.clone());
                Ok(actual)
            }
            Expr::Lambda(var, ty, expr) => {
                let mut new_type_infer = Self::from(TypeEnv::with_outer(Rc::clone(&self.env)));
                let nty = self.new_typevar();
                new_type_infer
                    .env
                    .borrow_mut()
                    .set(var.clone(), nty.clone());
                let ret_type = new_type_infer.infer_type(&expr)?;
                if ty.is_some() {
                    Self::unify(&ty.as_ref().unwrap(), &nty)?;
                }
                Ok(Type::func(nty.clone(), ret_type))
            }
            Expr::App(fun, var) => {
                let fun_type = self.infer_type(&fun)?;
                let var_type = self.infer_type(&var)?;
                if let Type::Func(dom, cod) = fun_type {
                    Self::unify(&dom, &var_type)?;
                    Ok(*cod)
                } else {
                    bail!("invalid func type")
                }
            }
        }
    }

    fn unify(t1: &Type, t2: &Type) -> Result<()> {
        match (t1, t2) {
            (Type::Bool, Type::Bool) => Ok(()),
            (Type::Int, Type::Int) => Ok(()),
            (Type::Func(arg1, ret1), Type::Func(arg2, ret2)) => {
                Self::unify(&arg1, &arg2)?;
                Self::unify(&ret1, &ret2)
            }
            (Type::TypeVar(id1, _), Type::TypeVar(id2, _)) if id1 == id2 => Ok(()),
            (Type::TypeVar(id1, t1), t2) => Self::unify_var(id1, t1, t2),
            (t1, Type::TypeVar(id2, t2)) => Self::unify_var(id2, t2, t1),
            (t1, t2) => bail!("unify error: connot unify {} and {}", t1, t2),
        }
    }

    fn unify_var(id1: &u64, tref1: &Rc<RefCell<Option<Type>>>, ty2: &Type) -> Result<()> {
        match ty2 {
            Type::TypeVar(_, tref2) => {
                if let Some(ref ty1) = *(*tref1).borrow() {
                    Self::unify(ty1, ty2)
                } else {
                    if let Some(ref ty2i) = *(*tref2).borrow() {
                        Self::unify_var(id1, tref1, ty2i)
                    } else {
                        *(*tref1).borrow_mut() = Some(ty2.clone());
                        Ok(())
                    }
                }
            }
            _ => {
                if Self::occur(id1, ty2) {
                    bail!("occue error")
                } else {
                    *(*tref1).borrow_mut() = Some(ty2.clone());
                    Ok(())
                }
            }
        }
    }

    fn occur(n: &u64, t: &Type) -> bool {
        match t {
            Type::Int => false,
            Type::Bool => false,
            Type::Func(arg, ret) => Self::occur(n, arg) || Self::occur(n, ret),
            Type::TypeVar(m, t1) => {
                if n == m {
                    return true;
                }

                match *(*t1).borrow() {
                    Some(ref t1) => Self::occur(n, &t1),
                    None => false,
                }
            }
        }
    }
}
