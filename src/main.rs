use ast::Expr;
use eval::Eval;
use parse::Parser;

mod ast;
mod eval;
mod parse;

fn main() {
    let expr: Expr = Parser::new("7*3+1+6/3*2").parse();
    dbg!(&expr.to_string());
    let ret = Eval::new().eval(expr);
    dbg!(&ret);
}
