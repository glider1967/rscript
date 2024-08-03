use core::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    Ident(String),
    Program(Vec<Expr>, Box<Expr>),
    BinPlus(Box<Expr>, Box<Expr>),
    BinMinus(Box<Expr>, Box<Expr>),
    BinMult(Box<Expr>, Box<Expr>),
    BinDiv(Box<Expr>, Box<Expr>),
    BinEq(Box<Expr>, Box<Expr>),
    BinNotEq(Box<Expr>, Box<Expr>),
    BinLT(Box<Expr>, Box<Expr>),
    BinGT(Box<Expr>, Box<Expr>),
    BinLE(Box<Expr>, Box<Expr>),
    BinGE(Box<Expr>, Box<Expr>),
    BinAnd(Box<Expr>, Box<Expr>),
    BinOr(Box<Expr>, Box<Expr>),
    UnaryMinus(Box<Expr>),
    UnaryNot(Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Assign(String, Box<Expr>),
    Lambda(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

macro_rules! expr_helpers {
    ($(($camel:ident, $snake:ident)($($arg:ident: $ty:ty),*)),*) => {
        impl Expr {
            $(
                pub fn $snake($($arg: $ty),*) -> Self {
                    Expr::$camel($(Box::new($arg)),*)
                }
            )*
        }
    }
}

// マクロを使用して関数を生成
expr_helpers! {
    (BinPlus, bin_plus)(left: Expr, right: Expr),
    (BinMinus, bin_minus)(left: Expr, right: Expr),
    (BinMult, bin_mult)(left: Expr, right: Expr),
    (BinDiv, bin_div)(left: Expr, right: Expr),
    (BinEq, bin_eq)(left: Expr, right: Expr),
    (BinNotEq, bin_neq)(left: Expr, right: Expr),
    (BinLT, bin_lt)(left: Expr, right: Expr),
    (BinGT, bin_gt)(left: Expr, right: Expr),
    (BinLE, bin_le)(left: Expr, right: Expr),
    (BinGE, bin_ge)(left: Expr, right: Expr),
    (BinAnd, bin_and)(left: Expr, right: Expr),
    (BinOr, bin_or)(left: Expr, right: Expr),
    (UnaryMinus, unary_minus)(expr: Expr),
    (UnaryNot, unary_not)(expr: Expr),
    (If, if_expr)(cond: Expr, expr: Expr, else_expr: Expr),
    (App, app)(fun: Expr, arg: Expr)
}

impl Expr {
    pub fn int(num: i64) -> Self {
        Expr::Int(num)
    }

    pub fn boolean(b: bool) -> Self {
        Expr::Bool(b)
    }

    pub fn ident(name: String) -> Self {
        Expr::Ident(name)
    }

    pub fn assign(name: String, expr: Expr) -> Self {
        Expr::Assign(name, Box::new(expr))
    }

    pub fn lambda(name: String, expr: Expr) -> Self {
        Expr::Lambda(name, Box::new(expr))
    }

    pub fn program(prog: Vec<Expr>, ret: Expr) -> Self {
        Expr::Program(prog, Box::new(ret))
    }

}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Int(v) => write!(f, "Int({})", v),
            Expr::Bool(v) => write!(f, "{}", v),
            Expr::Ident(name) => write!(f, "{}", name),
            Expr::Program(v, ret) => write!(f, 
                "{} {}",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                ret.to_string()
            ),
            Expr::BinPlus(exp1, exp2) => write!(f, "({} + {})", exp1.to_string(), exp2.to_string()),
            Expr::BinMinus(exp1, exp2) => write!(f, "({} - {})", exp1.to_string(), exp2.to_string()),
            Expr::BinMult(exp1, exp2) => write!(f, "({} * {})", exp1.to_string(), exp2.to_string()),
            Expr::BinDiv(exp1, exp2) => write!(f, "({} / {})", exp1.to_string(), exp2.to_string()),
            Expr::BinEq(exp1, exp2) => write!(f, "({} == {})", exp1.to_string(), exp2.to_string()),
            Expr::BinNotEq(exp1, exp2) => write!(f, "({} != {})", exp1.to_string(), exp2.to_string()),
            Expr::BinLT(exp1, exp2) => write!(f, "({} < {})", exp1.to_string(), exp2.to_string()),
            Expr::BinGT(exp1, exp2) => write!(f, "({} > {})", exp1.to_string(), exp2.to_string()),
            Expr::BinLE(exp1, exp2) => write!(f, "({} <= {})", exp1.to_string(), exp2.to_string()),
            Expr::BinGE(exp1, exp2) => write!(f, "({} >= {})", exp1.to_string(), exp2.to_string()),
            Expr::BinAnd(exp1, exp2) => write!(f, "({} && {})", exp1.to_string(), exp2.to_string()),
            Expr::BinOr(exp1, exp2) => write!(f, "({} || {})", exp1.to_string(), exp2.to_string()),
            Expr::UnaryMinus(exp1) => write!(f, "-{}", exp1.to_string()),
            Expr::UnaryNot(exp1) => write!(f, "!{}", exp1.to_string()),
            Expr::If(cond, exp1, exp2) => write!(f, 
                "if ({}) {{ {} }} else {{ {} }}",
                cond.to_string(),
                exp1.to_string(),
                exp2.to_string()
            ),
            Expr::Assign(ident, expr) => {
                write!(f, "let {ident} = {};", expr.to_string())
            }
            Expr::Lambda(var, _) => {
                write!(f, "<lambda ({})>", var)
            }
            Expr::App(fun, var) => {
                write!(f, "{}({})", fun.to_string(), var.to_string())
            }
        }
    }
}
