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
        let w = true;
        let f = lambda (w) {
            lambda (v) {
                let a = w*100;
                v
            }
        };
        f
        "#,
    )
    .prog()
    .context("Parse Error")?;
    dbg!(&stmt.to_string());

    dbg!(TypeInfer::new().infer_type(&stmt))?;

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
    dbg!(&stmt.to_string());
    // dbg!(TypeInfer::new().infer_type(&stmt)?);

    dbg!(Eval::new().eval(&stmt)?.to_string());
    Ok(())
}
