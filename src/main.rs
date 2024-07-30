use anyhow::{Ok, Result};
use eval::Eval;
use parse::Parser;

mod environment;
mod eval;
mod expression;
mod parse;
mod statement;
mod tokenize;

fn main() -> Result<()> {
    let stmt = Parser::new("let a = 2; let b = a + 1; a * b").parse_stmt()?;
    dbg!(&stmt.to_string());

    dbg!(Eval::new().eval_stmt(&stmt)?);
    Ok(())
}
