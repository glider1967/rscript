use anyhow::{Ok, Result};
use eval::Eval;
use parse::Parser;

mod environment;
mod eval;
mod expression;
mod parse;
mod tokenize;

fn main() -> Result<()> {
    let stmt = Parser::new("let f= lambda (v) {let a = v+1; a}; f(2)").prog()?;
    dbg!(&stmt.to_string());

    dbg!(Eval::new().eval_expr(&stmt)?.to_string());
    Ok(())
}
