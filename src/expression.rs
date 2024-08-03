use core::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    Ident(String),
    Program(Vec<Expr>, Box<Expr>),
    BinOp(String, Box<Expr>, Box<Expr>),
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

    pub fn binop(name: String, exp1: Expr, exp2: Expr) -> Self {
        Expr::BinOp(name, Box::new(exp1), Box::new(exp2))
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
            Expr::Program(v, ret) => write!(
                f,
                "{} {}",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                ret.to_string()
            ),
            Expr::BinOp(op, exp1, exp2) => write!(f, "({exp1} {op} {exp2})"),
            Expr::UnaryMinus(exp1) => write!(f, "-{}", exp1),
            Expr::UnaryNot(exp1) => write!(f, "!{}", exp1),
            Expr::If(cond, exp1, exp2) => {
                write!(f, "if ({}) {{ {} }} else {{ {} }}", cond, exp1, exp2)
            }
            Expr::Assign(ident, expr) => {
                write!(f, "let {ident} = {};", expr)
            }
            Expr::Lambda(var, _) => {
                write!(f, "<lambda ({})>", var)
            }
            Expr::App(fun, var) => {
                write!(f, "{}({})", fun, var)
            }
        }
    }
}
