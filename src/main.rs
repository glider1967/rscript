use eval::Eval;
use parse::Parser;

mod ast;
mod eval;
mod parse;
mod tokenize;

fn main() {
    let expr = Parser::new("1 < (5-3) <= 7 || !false && 7*3+2 > -18").parse();
    dbg!(&expr.to_string());

    dbg!(Eval::new().eval(expr));
}
