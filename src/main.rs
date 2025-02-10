use anyhow::{Context, Ok, Result};

use eval::Eval;
use parse::Parser;
use type_infer::TypeInfer;

mod environment;
mod eval;
mod expression;
mod internal_value;
mod parse;
mod span;
mod tokenize;
mod type_infer;
mod types;

fn main() -> Result<()> {
    let stmt = Parser::new(
        r#"
        let fizzbuzz = lambda(n: int) {
            if (n == 0) {
                ""
            } else {
                if (n % 15 == 0) {
                    fizzbuzz(n - 1) ++ "FizzBuzz\n"
                } else {
                    if (n % 5 == 0) {
                        fizzbuzz(n - 1) ++ "Buzz\n"
                    } else {
                        if (n % 3 == 0) {
                            fizzbuzz(n - 1) ++ "Fizz\n"
                        } else {
                            fizzbuzz(n - 1) ++ ~n ++ "\n"
                        }
                    }
                }
            }
        };
        fizzbuzz(30)
        "#,
    )
    .prog()
    .context("Parse Error")?;
    let string = &stmt.to_string();
    println!("{}", string);

    let inferred = TypeInfer::new()
        .infer_type(&stmt)
        .context("Type Inferrence Error")?;
    println!("inffered: {}", inferred);

    let evaluated = Eval::new().eval(&stmt).context("Evaluation Error")?;
    println!("{}", evaluated.to_string());
    Ok(())
}
