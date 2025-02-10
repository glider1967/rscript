use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::{bail, Ok, Result};
use thiserror::Error;

use crate::{
    expression::{ConstructorDef, Expr, Pattern, SpannedExpr},
    span::Span,
    types::Type,
};

#[derive(Error, Debug)]
pub enum TypeInferError {
    #[error("at {span}: connot unify {ty1} and {ty2}")]
    UnificationError {
        ty1: String,
        ty2: String,
        span: Span,
    },
}

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

    fn get(&self, name: &str) -> Result<Type> {
        if let Some(val) = self.env.get(name) {
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

    pub fn infer_type(&mut self, ast: &SpannedExpr) -> Result<Type> {
        let expr = &ast.expr;
        match expr {
            Expr::Int(_) => Ok(Type::constant("int")),
            Expr::Bool(_) => Ok(Type::constant("bool")),
            Expr::Str(_) => Ok(Type::constant("string")),
            Expr::Variable(name) => {
                let actual_type = self.env.borrow().get(name)?;
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
                "++" => {
                    let t1 = self.infer_type(&exp1)?;
                    let t2 = self.infer_type(&exp2)?;
                    Self::unify_str(&t1, &exp1)?;
                    Self::unify_str(&t2, &exp2)?;
                    Ok(Type::constant("string"))
                }
                "+" | "-" | "*" | "/" | "%" => {
                    let t1 = self.infer_type(&exp1)?;
                    let t2 = self.infer_type(&exp2)?;
                    Self::unify_int(&t1, &exp1)?;
                    Self::unify_int(&t2, &exp2)?;
                    Ok(Type::constant("int"))
                }
                "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                    let t1 = self.infer_type(&exp1)?;
                    let t2 = self.infer_type(&exp2)?;
                    Self::unify_int(&t1, &exp1)?;
                    Self::unify_int(&t2, &exp2)?;
                    Ok(Type::constant("bool"))
                }
                "&&" | "||" => {
                    let t1 = self.infer_type(&exp1)?;
                    let t2 = self.infer_type(&exp2)?;
                    Self::unify_bool(&t1, &exp1)?;
                    Self::unify_bool(&t2, &exp2)?;
                    Ok(Type::constant("bool"))
                }
                _ => bail!("invalid operator: {}", op),
            },
            Expr::UnaryOp(op, expr) => match op.as_str() {
                "-" => {
                    let t1 = self.infer_type(&expr)?;
                    Self::unify_int(&t1, &expr)?;
                    Ok(Type::constant("int"))
                }
                "!" => {
                    let t1 = self.infer_type(&expr)?;
                    Self::unify_bool(&t1, &expr)?;
                    Ok(Type::constant("bool"))
                }
                _ => bail!("invalid operator: {}", op),
            },
            Expr::If(cond, exp1, exp2) => {
                let t0 = self.infer_type(&cond)?;
                let t1 = self.infer_type(&exp1)?;
                let t2 = self.infer_type(&exp2)?;
                Self::unify_bool(&t0, &cond)?;
                Self::unify_error(&t1, &t2, &ast)?;
                Ok(t1)
            }
            Expr::Assign(ident, ty, expr) => {
                let nty = match ty {
                    Some(t) => t.clone(),
                    None => self.new_typevar(),
                };

                self.env.borrow_mut().set(ident.clone(), nty.clone());
                let actual = self.infer_type(&expr)?;

                if let Some(expected) = ty {
                    Self::unify_error(expected, &actual, &expr)?;
                }
                Self::unify_error(&nty, &actual, &ast)?;
                self.generalize(&actual);
                self.env.borrow_mut().set(ident.clone(), actual.clone());
                Ok(actual)
            }
            Expr::Reassign(ident, expr) => {
                let already = self.env.borrow().get(ident)?;
                let actual = self.infer_type(&expr)?;

                Self::unify_error(&already, &actual, &expr)?;
                Ok(actual)
            }
            Expr::EnumDef(ident, defs) => {
                for ConstructorDef { name, args } in defs {
                    self.env.borrow_mut().set(
                        name.to_owned(),
                        args.iter().rev().fold(Type::constant(&ident), |acc, ty| {
                            Type::func(ty.clone(), acc)
                        }),
                    );
                }
                Ok(Type::constant(&ident))
            }
            Expr::Match(expr, arms) => {
                let ty = self.infer_type(&expr)?;
                if arms.is_empty() {
                    return Ok(ty);
                }
                let ret_ty = self.new_typevar();
                for (pattern, expr) in arms {
                    let mut new_type_infer = Self::from(
                        TypeEnv::with_outer(Rc::clone(&self.env)),
                        self.next_typevar_id,
                    );
                    new_type_infer.infer_pattern(&pattern, &ty)?;
                    Self::unify(&ret_ty, &new_type_infer.infer_type(expr)?)?;
                    self.next_typevar_id = new_type_infer.next_typevar_id;
                }
                Ok(ret_ty)
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
                if ty.is_some() {
                    Self::unify_error(&ty.as_ref().unwrap(), &nty, &ast)?;
                }
                self.next_typevar_id = new_type_infer.next_typevar_id;
                Ok(Type::func(nty, ret_type))
            }
            Expr::App(fun, var) => {
                let fun_type = self.infer_type(&fun)?;
                let var_type = self.infer_type(&var)?;
                let nty = self.new_typevar();
                Self::unify_error(&fun_type, &Type::func(var_type, nty.clone()), &ast)?;
                Ok(nty)
            }
        }
    }

    fn infer_pattern(&mut self, pattern: &Pattern, ret_ty: &Type) -> Result<()> {
        let constr_ty = self.env.borrow().get(&pattern.name)?;
        let (args, rty) = constr_ty.to_args_and_ret();
        if args.len() != pattern.vars.len() {
            bail!("number of pattern variable is invalid");
        }
        for i in 0..args.len() {
            let nty = self.new_typevar();
            Self::unify(&nty, &args[i])?;
            self.env.borrow_mut().set(pattern.vars[i].to_owned(), nty);
        }
        Self::unify(&rty, ret_ty)?;
        Ok(())
    }

    // 単一化 - ”型のつじつま合わせ”
    fn unify(t1: &Type, t2: &Type) -> Result<()> {
        match (Self::zonk(t1), Self::zonk(t2)) {
            (Type::Constant(c1), Type::Constant(c2)) if c1 == c2 => Ok(()),
            (Type::Func(arg1, ret1), Type::Func(arg2, ret2)) => {
                Self::unify(&arg1, &arg2)?;
                Self::unify(&ret1, &ret2)
            }
            (Type::TypeVar(id1, _), Type::TypeVar(id2, _)) if id1 == id2 => Ok(()),
            (Type::TypeVar(id1, t1), t2) => Self::unify_var(&id1, &t1, &t2),
            (t1, Type::TypeVar(id2, t2)) => Self::unify_var(&id2, &t2, &t1),
            _ => bail!("unify error"),
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

    fn unify_int(ty: &Type, expr: &SpannedExpr) -> Result<()> {
        Self::unify(&ty, &Type::constant("int")).map_err(|_| {
            TypeInferError::UnificationError {
                ty1: ty.to_string(),
                ty2: "int".to_string(),
                span: expr.span.clone(),
            }
            .into()
        })
    }

    fn unify_bool(ty: &Type, expr: &SpannedExpr) -> Result<()> {
        Self::unify(&ty, &Type::constant("bool")).map_err(|_| {
            TypeInferError::UnificationError {
                ty1: ty.to_string(),
                ty2: "bool".to_string(),
                span: expr.span.clone(),
            }
            .into()
        })
    }

    fn unify_str(ty: &Type, expr: &SpannedExpr) -> Result<()> {
        Self::unify(&ty, &Type::constant("string")).map_err(|_| {
            TypeInferError::UnificationError {
                ty1: ty.to_string(),
                ty2: "string".to_string(),
                span: expr.span.clone(),
            }
            .into()
        })
    }

    fn unify_error(ty1: &Type, ty2: &Type, expr: &SpannedExpr) -> Result<()> {
        Self::unify(&ty1, &ty2).map_err(|_| {
            TypeInferError::UnificationError {
                ty1: ty1.to_string(),
                ty2: ty2.to_string(),
                span: expr.span.clone(),
            }
            .into()
        })
    }

    // 型変数の出現チェック
    fn occur(n: &u64, t: &Type) -> bool {
        match Self::zonk(t) {
            Type::Constant(_) => false,
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
            Type::Constant(s) => Type::Constant(s.clone()),
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
            Type::Constant(_) => (),
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
            Type::Constant(s) => Type::Constant(s.clone()),
            Type::Func(t1, t2) => Type::func(self.inst_inner(*t1, map), self.inst_inner(*t2, map)),
            Type::TypeVar(_, _) => ty,
        }
    }
}
