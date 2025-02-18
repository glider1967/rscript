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
mod type_env;
mod type_infer;
mod types;

fn main() -> Result<()> {
    let stmt = Parser::new(
        r#"
enum List {
    Cons(int, List),
    Nil
};
let sum = lambda(l: List) {
    match (l) {
        Cons(x, xs) => x + sum(xs),
        Nil => 0
    }
};
let map = lambda(l: List, f: int -> int) {
    match(l) {
        Cons(x, xs) => Cons(f(x), map(xs, f)),
        Nil => Nil
    }
};
let l = Cons(1, Cons(2, Cons(3, Nil)));
sum(map(l, lambda(x){x*x})) // 1*1 + 2*2 + 3*3
        "#,
    )
    .prog()
    .context("Parse Error")?;
    let string = &stmt.to_string();
    println!("{}", string);

    let mut tyinf = TypeInfer::new();
    let inferred = tyinf.infer_type(&stmt).context("Type Inferrence Error")?;
    println!("inffered: {}", inferred);
    println!("type environment:\n {}", tyinf);

    let evaluated = Eval::new().eval(&stmt).context("Evaluation Error")?;
    println!("{}", evaluated);
    Ok(())
}
