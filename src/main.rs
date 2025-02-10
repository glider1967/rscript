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
        enum List {
            Cons(int, List),
            Nil
        };
        let len = lambda(l: List) {
            match (l) {
                Cons(x, y) => x + len(y),
                Nil => 0
            }
        };
        let s: string = "sgt hjk <\n\"\\<<< " ++ "p";
        s
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

    // let stmt = Parser::new(
    //     r#"
    //     let f = lambda (n) {
    //         if(n == 1 || n == 2) { 1 } else { f(n-1) + f(n-2) }
    //     };
    //     f(10)
    //     "#,
    // )
    // .prog()?;
    // dbg!(&stmt.to_string());
    // dbg!(TypeInfer::new().infer_type(&stmt)?);

    // dbg!(Eval::new().eval(&stmt)?.to_string());
    Ok(())
}
