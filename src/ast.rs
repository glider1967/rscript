#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Int(u64),
    BinPlus(Box<Expr>, Box<Expr>),
    BinMinus(Box<Expr>, Box<Expr>),
    BinMult(Box<Expr>, Box<Expr>),
    BinDiv(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn int(num: u64) -> Self {
        Expr::Int(num)
    }

    pub fn plus(exp1: Expr, exp2: Expr) -> Self {
        Expr::BinPlus(Box::new(exp1), Box::new(exp2))
    }
}
