use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::{bail, Result};

use crate::{expression::Expr, types::Type};

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

    fn from(env: TypeEnv, next: u64) -> Self {
        Self {
            env: Rc::new(RefCell::new(env)),
            next_typevar_id: next,
        }
    }

    // 新しい名前の型変数を用意
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
                Ok(self.instantiate(&actual_type))
            }
            Expr::Program(v, ret) => {
                for expr in v {
                    let _ = self.infer_type(expr)?;
                    // println!("{}", t);
                }
                let ret_type = self.infer_type(&ret)?;
                Ok(ret_type)
            }
            Expr::BinOp(op, exp1, exp2) => match op.as_str() {
                "+" | "-" | "*" | "/" => {
                    let t1 = self.infer_type(&exp1)?;
                    let t2 = self.infer_type(&exp2)?;
                    Self::unify(&t1, &Type::Int)?;
                    Self::unify(&t2, &Type::Int)?;
                    Ok(Type::Int)
                }
                "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                    let t1 = self.infer_type(&exp1)?;
                    let t2 = self.infer_type(&exp2)?;
                    Self::unify(&t1, &Type::Int)?;
                    Self::unify(&t2, &Type::Int)?;
                    Ok(Type::Int)
                }
                "&&" | "||" => {
                    let t1 = self.infer_type(&exp1)?;
                    let t2 = self.infer_type(&exp2)?;
                    Self::unify(&t1, &Type::Bool)?;
                    Self::unify(&t2, &Type::Bool)?;
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
                let nty = match ty {
                    Some(t) => t.clone(),
                    None => self.new_typevar(),
                };

                self.env.borrow_mut().set(ident.clone(), nty);
                let actual = self.infer_type(&expr)?;

                if let Some(expected) = ty {
                    Self::unify(expected, &actual)?;
                }
                self.generalize(&actual);
                self.env.borrow_mut().set(ident.clone(), actual.clone());
                Ok(actual)
            }
            Expr::Lambda(var, ty, expr) => {
                let nty = self.new_typevar();
                let mut new_type_infer = Self::from(
                    TypeEnv::with_outer(Rc::clone(&self.env)),
                    self.next_typevar_id,
                );
                new_type_infer
                    .env
                    .borrow_mut()
                    .set(var.clone(), nty.clone());
                let ret_type = new_type_infer.infer_type(&expr)?;
                self.next_typevar_id = new_type_infer.next_typevar_id;
                if ty.is_some() {
                    Self::unify(&ty.as_ref().unwrap(), &nty)?;
                }
                Ok(Type::func(nty, ret_type))
            }
            Expr::App(fun, var) => {
                let fun_type = self.infer_type(&fun)?;
                let var_type = self.infer_type(&var)?;
                let nty = self.new_typevar();
                Self::unify(&fun_type, &Type::func(var_type, nty.clone()))?;
                Ok(nty)
            }
        }
    }

    // 単一化 - ”型のつじつま合わせ”
    fn unify(t1: &Type, t2: &Type) -> Result<()> {
        match (Self::zonk(t1), Self::zonk(t2)) {
            (Type::Bool, Type::Bool) => Ok(()),
            (Type::Int, Type::Int) => Ok(()),
            (Type::Func(arg1, ret1), Type::Func(arg2, ret2)) => {
                Self::unify(&arg1, &arg2)?;
                Self::unify(&ret1, &ret2)
            }
            (Type::TypeVar(id1, _), Type::TypeVar(id2, _)) if id1 == id2 => Ok(()),
            (Type::TypeVar(id1, t1), t2) => Self::unify_var(&id1, &t1, &t2),
            (t1, Type::TypeVar(id2, t2)) => Self::unify_var(&id2, &t2, &t1),
            (t1, t2) => bail!("unify error: connot unify {} and {}", t1, t2),
        }
    }

    fn unify_var(id1: &u64, tref1: &Rc<RefCell<Option<Type>>>, ty2: &Type) -> Result<()> {
        if Self::occur(id1, ty2) {
            bail!("occur error")
        } else {
            *(*tref1).borrow_mut() = Some(ty2.clone());
            Ok(())
        }
    }

    // 型変数の出現チェック
    fn occur(n: &u64, t: &Type) -> bool {
        match Self::zonk(t) {
            Type::Int => false,
            Type::Bool => false,
            Type::Func(arg, ret) => Self::occur(n, &arg) || Self::occur(n, &ret),
            Type::TypeVar(m, t1) => {
                if *n == m {
                    return true;
                }

                match *(*t1).borrow() {
                    Some(ref t1) => Self::occur(n, &t1),
                    None => false,
                }
            }
            Type::Quantifier(_) => false,
        }
    }

    // zonking - 確定した型変数を引きはがす
    fn zonk(t: &Type) -> Type {
        match t {
            Type::Int => Type::Int,
            Type::Bool => Type::Bool,
            Type::Func(arg, ret) => Type::func(Self::zonk(arg), Self::zonk(ret)),
            Type::TypeVar(_, t1) => match *(*t1).borrow() {
                Some(ref t1) => Self::zonk(t1),
                None => t.clone(),
            },
            Type::Quantifier(id) => Type::Quantifier(*id),
        }
    }

    // 一般化 - 推論されなかった型変数を量化
    fn generalize(&self, t: &Type) {
        match Self::zonk(t) {
            Type::Int => (),
            Type::Bool => (),
            Type::Func(arg, ret) => {
                self.generalize(&*arg);
                self.generalize(&*ret);
            }
            Type::TypeVar(n, t1) => *(*t1).borrow_mut() = Some(Type::Quantifier(n)),
            Type::Quantifier(_) => (),
        }
    }

    // インスタンス化 - 量化された型変数を再び普通の型変数に
    fn instantiate(&mut self, t: &Type) -> Type {
        let ty = Self::zonk(t);
        self.inst_inner(ty, &mut HashMap::new())
    }

    fn inst_inner(&mut self, ty: Type, map: &mut HashMap<u64, Type>) -> Type {
        match ty {
            Type::Quantifier(n) => match map.get(&n) {
                Some(t) => t.clone(),
                None => {
                    let ty = self.new_typevar();
                    map.insert(n, ty.clone());
                    ty
                }
            },
            Type::Int => Type::Int,
            Type::Bool => Type::Bool,
            Type::Func(t1, t2) => Type::func(self.inst_inner(*t1, map), self.inst_inner(*t2, map)),
            Type::TypeVar(_, _) => ty,
        }
    }
}
