use eval::Eval;
use parse::Parser;
use serde::Serialize;
use tsify::Tsify;
use type_infer::TypeInfer;
use wasm_bindgen::prelude::*;

mod environment;
mod eval;
mod expression;
mod internal_value;
mod parse;
mod tokenize;
mod type_infer;
mod types;

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
pub enum EvaluationResult {
    Ok(String),
    ParseError(String),
    TypeInferError(String),
    EvaluationError(String),
}

#[wasm_bindgen]
pub fn eval_script(line: &str) -> EvaluationResult {
    match Parser::new(line).prog() {
        Ok(stmt) => match TypeInfer::new().infer_type(&stmt) {
            Ok(_) => match Eval::new().eval(&stmt) {
                Ok(val) => EvaluationResult::Ok(val.to_string()),
                Err(err) => EvaluationResult::EvaluationError(err.to_string()),
            },
            Err(err) => EvaluationResult::TypeInferError(err.to_string()),
        },
        Err(err) => EvaluationResult::ParseError(err.to_string()),
    }
}
