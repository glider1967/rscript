use anyhow::{Context, Ok, Result};

use eval::Eval;
use parse::Parser;
use type_infer::TypeInfer;

mod environment;
mod eval;
mod expression;
mod internal_value;
mod parse;
mod tokenize;
mod type_infer;
mod types;

fn main() -> Result<()> {
    let stmt = Parser::new(
        r#"
        let w = true;
        let f: (int -> int) -> int -> int = lambda (w) {
            lambda(x) {
                x
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
        let f = lambda (n) {
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
