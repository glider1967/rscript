use anyhow::{Ok, Result};
use eval::Eval;
use parse::Parser;

mod environment;
mod eval;
mod expression;
mod parse;
mod tokenize;

fn main() -> Result<()> {
    let stmt = Parser::new("let a = 2; let b = a + 1; a * b").statement()?;
    dbg!(&stmt.iter().map(|x| x.to_string()).collect::<Vec<_>>());

    dbg!(Eval::new().eval(&stmt)?);
    Ok(())
}
