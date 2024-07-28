use ast::Expr;
use parse::Parser;

mod ast;
mod parse;
fn main() {
    let expr: Expr = Parser::new("2+3+4+(4+1)").parse();
    dbg!(expr);
}
