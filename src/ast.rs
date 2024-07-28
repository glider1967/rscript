#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    BinPlus(Box<Expr>, Box<Expr>),
    BinMinus(Box<Expr>, Box<Expr>),
    BinMult(Box<Expr>, Box<Expr>),
    BinDiv(Box<Expr>, Box<Expr>),
    BinEq(Box<Expr>, Box<Expr>),
    BinLT(Box<Expr>, Box<Expr>),
    BinGT(Box<Expr>, Box<Expr>),
    BinLE(Box<Expr>, Box<Expr>),
    BinGE(Box<Expr>, Box<Expr>),
    BinAnd(Box<Expr>, Box<Expr>),
    BinOr(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn int(num: i64) -> Self {
        Expr::Int(num)
    }

    pub fn plus(exp1: Expr, exp2: Expr) -> Self {
        Expr::BinPlus(Box::new(exp1), Box::new(exp2))
    }

    pub fn minus(exp1: Expr, exp2: Expr) -> Self {
        Expr::BinMinus(Box::new(exp1), Box::new(exp2))
    }

    pub fn mult(exp1: Expr, exp2: Expr) -> Self {
        Expr::BinMult(Box::new(exp1), Box::new(exp2))
    }

    pub fn div(exp1: Expr, exp2: Expr) -> Self {
        Expr::BinDiv(Box::new(exp1), Box::new(exp2))
    }

    pub fn to_string(&self) -> String {
        match self {
            Expr::Int(v) => format!("Int({})", v),
            Expr::Bool(v) => format!("{}", v),
            Expr::BinPlus(exp1, exp2) => format!("({} + {})", exp1.to_string(), exp2.to_string()),
            Expr::BinMinus(exp1, exp2) => format!("({} - {})", exp1.to_string(), exp2.to_string()),
            Expr::BinMult(exp1, exp2) => format!("({} * {})", exp1.to_string(), exp2.to_string()),
            Expr::BinDiv(exp1, exp2) => format!("({} / {})", exp1.to_string(), exp2.to_string()),
            Expr::BinEq(exp1, exp2) => format!("({} == {})", exp1.to_string(), exp2.to_string()),
            Expr::BinLT(exp1, exp2) => format!("({} < {})", exp1.to_string(), exp2.to_string()),
            Expr::BinGT(exp1, exp2) => format!("({} > {})", exp1.to_string(), exp2.to_string()),
            Expr::BinLE(exp1, exp2) => format!("({} <= {})", exp1.to_string(), exp2.to_string()),
            Expr::BinGE(exp1, exp2) => format!("({} >= {})", exp1.to_string(), exp2.to_string()),
            Expr::BinAnd(exp1, exp2) => format!("({} && {})", exp1.to_string(), exp2.to_string()),
            Expr::BinOr(exp1, exp2) => format!("({} || {})", exp1.to_string(), exp2.to_string()),
        }
    }
}
