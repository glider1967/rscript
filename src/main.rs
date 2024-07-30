use anyhow::{Ok, Result};
use eval::Eval;
use parse::Parser;

mod ast;
mod eval;
mod parse;
mod statement;
mod tokenize;

fn main() -> Result<()> {
    let expr = Parser::new("if(1<=2 || (if(true){false else{1>=2})){1}else{2}").parse()?;
    dbg!(&expr.to_string());

    dbg!(Eval::new().eval(expr)?);
    Ok(())
}
