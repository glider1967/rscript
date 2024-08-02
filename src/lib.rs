use eval::Eval;
use parse::Parser;
use wasm_bindgen::prelude::*;

mod environment;
mod eval;
mod expression;
mod parse;
mod tokenize;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn eval_script(line: &str) -> JsValue {
    match Parser::new(line).prog() {
        Ok(stmt) => {
            match Eval::new().eval(&stmt) {
                Ok(val) => val.to_string().into(),
                Err(err) => err.to_string().into()
            }
        }
        Err(err) => {
            err.to_string().into()
        }
    }
}
