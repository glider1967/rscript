use ast::Expr;
use eval::Eval;
use parse::Parser;

mod ast;
mod eval;
mod parse;

fn main() {
    let expr: Expr = Parser::new("7+(3-(4))-(4-1)").parse();
    dbg!(&expr);
    let ret = Eval::new().eval(expr);
    dbg!(&ret);
}
