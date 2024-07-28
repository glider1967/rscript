use crate::ast::Expr;

pub struct Eval;

impl Eval {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval(&self, ast: Expr) -> i64 {
        match ast {
            Expr::Int(v) => v,
            Expr::BinPlus(exp1, exp2) => self.eval(*exp1) + self.eval(*exp2),
            Expr::BinMinus(exp1, exp2) => self.eval(*exp1) - self.eval(*exp2),
            Expr::BinMult(exp1, exp2) => self.eval(*exp1) * self.eval(*exp2),
            Expr::BinDiv(exp1, exp2) => self.eval(*exp1) / self.eval(*exp2),
        }
    }
}
