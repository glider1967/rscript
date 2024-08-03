use anyhow::{Context, Ok, Result};

use eval::Eval;
use parse::Parser;
use types::TypeInfer;

mod environment;
mod eval;
mod expression;
mod internal_value;
mod parse;
mod tokenize;
mod types;

fn main() -> Result<()> {
    let stmt = Parser::new(
        r#"
        let f: int -> int -> int = lambda (w: int) {
            lambda (v: int) {
                let a: int = w*100;
                a + v
            }
        };
        f(2)(7)
        "#,
    )
    .prog()
    .context("Parse Error")?;
    dbg!(&stmt.expr.to_string());

    dbg!(TypeInfer::new().infer_type(&stmt)?);

    dbg!(Eval::new()
        .eval(&stmt)
        .context("Evaluation Error")?
        .to_string());

    let stmt = Parser::new(
        r#"
        let f: int -> int = lambda (n: int) {
            if(n == 1 || n == 2) { 1 } else { f(n-1) + f(n-2) }
        };
        f(10)
        "#,
    )
    .prog()?;
    dbg!(&stmt.expr.to_string());
    // dbg!(TypeInfer::new().infer_type(&stmt)?);

    dbg!(Eval::new().eval(&stmt)?.to_string());
    Ok(())
}
