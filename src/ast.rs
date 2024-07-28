#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Int(u64),
    BinPlus(Box<Expr>, Box<Expr>),
    BinMinus(Box<Expr>, Box<Expr>),
    BinMult(Box<Expr>, Box<Expr>),
    BinDiv(Box<Expr>, Box<Expr>),
}
