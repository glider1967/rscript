use crate::ast::Expr;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
}

pub struct Eval;

impl Eval {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval(&self, ast: Expr) -> Value {
        match ast {
            Expr::Int(v) => Value::Int(v),
            Expr::Bool(v) => Value::Bool(v),
            Expr::BinPlus(exp1, exp2) => {
                int_bin_op(Box::new(|x, y| x + y), self.eval(*exp1), self.eval(*exp2))
            }
            Expr::BinMinus(exp1, exp2) => {
                int_bin_op(Box::new(|x, y| x - y), self.eval(*exp1), self.eval(*exp2))
            }
            Expr::BinMult(exp1, exp2) => {
                int_bin_op(Box::new(|x, y| x * y), self.eval(*exp1), self.eval(*exp2))
            }
            Expr::BinDiv(exp1, exp2) => {
                int_bin_op(Box::new(|x, y| x / y), self.eval(*exp1), self.eval(*exp2))
            }
            Expr::BinEq(exp1, exp2) => {
                int_bin_op_bool(Box::new(|x, y| x == y), self.eval(*exp1), self.eval(*exp2))
            }
            Expr::BinNotEq(exp1, exp2) => {
                int_bin_op_bool(Box::new(|x, y| x != y), self.eval(*exp1), self.eval(*exp2))
            }
            Expr::BinLT(exp1, exp2) => {
                int_bin_op_bool(Box::new(|x, y| x < y), self.eval(*exp1), self.eval(*exp2))
            }
            Expr::BinGT(exp1, exp2) => {
                int_bin_op_bool(Box::new(|x, y| x > y), self.eval(*exp1), self.eval(*exp2))
            }
            Expr::BinLE(exp1, exp2) => {
                int_bin_op_bool(Box::new(|x, y| x <= y), self.eval(*exp1), self.eval(*exp2))
            }
            Expr::BinGE(exp1, exp2) => {
                int_bin_op_bool(Box::new(|x, y| x >= y), self.eval(*exp1), self.eval(*exp2))
            }
            Expr::BinAnd(exp1, exp2) => {
                bool_bin_op(Box::new(|x, y| x && y), self.eval(*exp1), self.eval(*exp2))
            }
            Expr::BinOr(exp1, exp2) => {
                bool_bin_op(Box::new(|x, y| x || y), self.eval(*exp1), self.eval(*exp2))
            }
            Expr::UnaryMinus(exp1) => int_unary_op(Box::new(|x: i64| -x), self.eval(*exp1)),
            Expr::UnaryNot(exp1) => bool_unary_op(Box::new(|x: bool| !x), self.eval(*exp1)),
        }
    }
}

fn int_bin_op<F>(func: F, v1: Value, v2: Value) -> Value
where
    F: Fn(i64, i64) -> i64,
{
    match (v1, v2) {
        (Value::Int(a), Value::Int(b)) => Value::Int(func(a, b)),
        _ => panic!("int binop for non-integer!"),
    }
}

fn int_bin_op_bool<F>(func: F, v1: Value, v2: Value) -> Value
where
    F: Fn(i64, i64) -> bool,
{
    match (v1, v2) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(func(a, b)),
        _ => panic!("int binop for non-integer!"),
    }
}

fn bool_bin_op<F>(func: F, v1: Value, v2: Value) -> Value
where
    F: Fn(bool, bool) -> bool,
{
    match (v1, v2) {
        (Value::Bool(a), Value::Bool(b)) => Value::Bool(func(a, b)),
        _ => panic!("bool binop for non-bool!"),
    }
}

fn int_unary_op<F>(func: F, v1: Value) -> Value
where
    F: Fn(i64) -> i64,
{
    match v1 {
        Value::Int(a) => Value::Int(func(a)),
        _ => panic!("int unary for non-integer!"),
    }
}

fn bool_unary_op<F>(func: F, v1: Value) -> Value
where
    F: Fn(bool) -> bool,
{
    match v1 {
        Value::Bool(a) => Value::Bool(func(a)),
        _ => panic!("bool unary for non-bool!"),
    }
}
