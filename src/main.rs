use eval::Eval;
use parse::Parser;
use tokenize::Tokenizer;

mod ast;
mod eval;
mod parse;
mod tokenize;

fn main() {
    let tokenizer = Tokenizer::new("1 +  356-6 <5<=7!!=").tokenize();
    dbg!(tokenizer);

    let expr = Parser::new("1+(2+3*6 - 2) *   2").parse();
    dbg!(&expr.to_string());

    dbg!(Eval::new().eval(expr));
}
