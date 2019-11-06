use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EvaluationResult {
    pub success: bool,
    pub value: i64,
    error: String,
}

#[wasm_bindgen]
impl EvaluationResult {
    // We need to expose strings via a getter so wasm_bindgen can do its magic,
    // since WASM doesn't have a native representation for strings.
    #[wasm_bindgen(getter)]
    pub fn error(&self) -> String {
        return self.error.clone();
    }
}

impl EvaluationResult
{
    pub fn from_error(error: String) -> EvaluationResult {
        return EvaluationResult {
            value: 0,
            success: false,
            error,
        }
    }

    pub fn from_result(result: i64) -> EvaluationResult {
        return EvaluationResult {
            value: result,
            success: true,
            error: String::default(),
        }
    }
}