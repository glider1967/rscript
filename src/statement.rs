use crate::ast::Expr;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Statement {
    Assign(String, Expr),
}
