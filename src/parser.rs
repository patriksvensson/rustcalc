use super::tokenizer::{Token, OperatorKind};
use super::RustCalcResult;

pub trait Visitor {
    fn visit_integer(&self, node: &IntegerExpression) -> i64;
    fn visit_arithmetic(&self, node: &ArithmeticExpression) -> i64;
}

pub trait Expression {
    fn accept(&self, visitor: &dyn Visitor) -> i64;
}

// Integer expression
pub struct IntegerExpression {
    pub value: i64
}
impl Expression for IntegerExpression {
    fn accept(&self, visitor: &dyn Visitor) -> i64 { 
        return visitor.visit_integer(self);
    }
}

// Arithmetic expression
pub struct ArithmeticExpression {
    pub operator: OperatorKind,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>
}
impl Expression for ArithmeticExpression {
    fn accept(&self, visitor: &dyn Visitor) -> i64 {
        return  visitor.visit_arithmetic(self);
    }
}

pub fn parse(tokens: Vec<Token>) -> RustCalcResult<Box<dyn Expression>> {
    let mut stack: Vec<Box<dyn Expression>> = Vec::new();
    for token in tokens {
        match token {
            Token::Integer(value) => stack.push(Box::new(IntegerExpression { value: value})),
            Token::LParen => { },
            Token::RParen => { },
            Token::Operator(op) => {
                let right = match stack.pop() {
                        Some(item) => item,
                        None => return Err(format_err!("Expected operand on stack."))
                };
                let left = match stack.pop() {
                        Some(item) => item,
                        None => return Err(format_err!("Expected operand on stack."))
                };
                stack.push(Box::new(ArithmeticExpression {
                    operator: op.kind,
                    left,
                    right
                }));
            }
        }
    }

    if stack.is_empty() {
        return Err(format_err!("Expected expression on stack but found none."));
    }

    return Ok(stack.pop().unwrap());
}