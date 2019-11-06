#[macro_use]
extern crate failure;

mod tokenizer;
mod parser;
mod result;

use failure::Error;
use parser::*;
use result::EvaluationResult;
use tokenizer::OperatorKind;
use wasm_bindgen::prelude::*;

pub type RustCalcResult<T> = Result<T, Error>;

#[wasm_bindgen]
pub fn evaluate(text: &str) -> EvaluationResult {
    // Empty expression?
    if text.is_empty() {
        return EvaluationResult::from_result(0);
    }

    // Tokenize the expression
    let tokens = match tokenizer::tokenize(text) {
        Ok(tokens) => tokens,
        Err(err) => return EvaluationResult::from_error(err.to_string()),
    };

    // Parse the expression
    let expression = match parser::parse(tokens)
    {
        Ok(expression) => expression,
        Err(err) => return EvaluationResult::from_error(err.to_string())
    };

    // Evaluate the expression
    let evaluator = Evaluator {};
    let result = expression.accept(&evaluator);

    // Return the result.
    return EvaluationResult::from_result(result);
}

// Evaluator
struct Evaluator { }
impl Visitor for Evaluator {
    fn visit_integer(&self, node: &IntegerExpression) -> i64 {
        return node.value;
    }
    fn visit_arithmetic(&self, node: &ArithmeticExpression) -> i64 {
        let left = node.left.accept(self);
        let right = node.right.accept(self);
        return match node.operator {
            OperatorKind::Addition => left + right,
            OperatorKind::Subtraction => left - right,
            OperatorKind::Multiplication => left * right,
            OperatorKind::Division => left / right,
            OperatorKind::Power => left.pow(right as u32),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_two_numbers_correctly() {
        let result = evaluate(&String::from("1+1-2+10"));
        assert_eq!(result.success, true);
        assert_eq!(result.value, 10);
    }

    #[test]
    fn should_respect_operator_precedence() {
        let result = evaluate(&String::from("1+9*9"));
        assert_eq!(result.success, true);
        assert_eq!(result.value, 82);
    }

    #[test]
    fn should_respect_parentheses() {
        let result = evaluate(&String::from("(1+9)*9"));
        assert_eq!(result.success, true);
        assert_eq!(result.value, 90);
    }

    #[test]
    fn should_respect_associativeness() {
        let result = evaluate(&String::from("2*2^3^2"));
        assert_eq!(result.success, true);
        assert_eq!(result.value, 1024);
    }

    #[test]
    fn should_return_error_if_operand_is_missing_from_stack_when_parsing_arithmetic_expression() {
        let result = evaluate(&String::from("^1+9*9"));
        assert_eq!(result.success, false);
        assert_eq!(result.error(), "Expected operand on stack.");
    }

    #[test]
    fn should_return_zero_for_empty_expression() {
        let result = evaluate(&String::from(""));
        assert_eq!(result.success, true);
        assert_eq!(result.value, 0);
    }
}


