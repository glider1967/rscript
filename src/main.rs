use anyhow::{Ok, Result};

use eval::Eval;
use parse::Parser;

mod environment;
mod eval;
mod expression;
mod internal_value;
mod parse;
mod tokenize;
mod types;

fn main() -> Result<()> {
    let stmt = Parser::new(
        "let q: int = 9; let f : int -> int -> int = lambda (w: int) {lambda (v: int) {let a: int = v*q; a + q}}; f(2)(100)",
    )
    .prog()?;
    dbg!(&stmt.expr.to_string());

    dbg!(Eval::new().eval(&stmt)?.to_string());

    let stmt = Parser::new(
        "let f: int -> int = lambda (n: int) { if(n == 1 || n == 2) {1} else {f(n-1) + f(n-2)} }; f(10)",
    )
    .prog()?;
    dbg!(&stmt.expr.to_string());

    dbg!(Eval::new().eval(&stmt)?.to_string());
    Ok(())
}
