use anyhow::{Ok, Result};
use eval::Eval;
use parse::Parser;

mod environment;
mod eval;
mod expression;
mod parse;
mod tokenize;

fn main() -> Result<()> {
    let stmt =
        Parser::new("let f= lambda (w) {lambda (v) {let a = v+1; a + w}}; f(2)(100)").prog()?;
    dbg!(&stmt.to_string());

    dbg!(Eval::new().eval_expr(&stmt)?.to_string());

    let stmt =
        Parser::new("let f= lambda (n) { if(n == 1 || n == 2) {1} else {f(n-1) + f(n-2)} }; f(10)")
            .prog()?;
    dbg!(&stmt.to_string());

    dbg!(Eval::new().eval_expr(&stmt)?.to_string());
    Ok(())
}
