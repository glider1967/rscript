use eval::Eval;
use parse::Parser;

mod ast;
mod eval;
mod parse;
mod tokenize;

fn main() {
    let expr = Parser::new("(1 + (3-1))>=7").parse();
    dbg!(&expr.to_string());

    dbg!(Eval::new().eval(expr));
}
