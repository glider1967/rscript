use crate::expression::Expr;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Statement {
    AssignAndConseq(String, Expr, Box<Statement>),
    Expression(Expr),
}

impl Statement {
    pub fn to_string(&self) -> String {
        match self {
            Self::AssignAndConseq(ident, expr, conseq) => {
                format!("let {ident} = {}; {}", expr.to_string(), conseq.to_string())
            }
            Self::Expression(expr) => expr.to_string(),
        }
    }
}
