use std::iter::Peekable;
use std::str::Chars;
use std::collections::VecDeque;
use super::RustCalcResult;

#[derive(Debug)]
pub enum Token {
    Integer(i64),
    Operator(Operator),
    LParen,
    RParen
}

impl Token {
    fn is_operator(&self) -> bool {
        return match self {
            Token::Operator(_) => true,
            _ => false,
        }
    }
    fn is_left_associative(&self) -> bool {
        return match self {
            Token::Operator(op) => op.is_left_associative,
            _ => false,
        }
    }
    fn get_precedence(&self) -> u8 {
        return match self {
            Token::Operator(op) => op.precedence,
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub struct Operator {
    pub kind: OperatorKind,
    precedence: u8,
    is_left_associative: bool
}

#[derive(Debug)]
pub enum OperatorKind {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Power
}

pub fn tokenize(text: &str) -> RustCalcResult<Vec<Token>> {
    let mut result : Vec<Token> = Vec::new();
    let mut stream = text.chars().peekable();
    loop {
        match stream.peek() {
            None => break,
            Some(&character) => match character {
                '0' ..= '9' => {
                    let number = parse_integer(&mut stream)?;
                    result.push(Token::Integer(number))
                },
                '+' => {
                    result.push(Token::Operator(Operator { 
                        kind: OperatorKind::Addition, 
                        precedence: 2,
                        is_left_associative: true,
                    }));
                    stream.next();
                },
                '-' => {
                    result.push(Token::Operator(Operator { 
                        kind: OperatorKind::Subtraction, 
                        precedence: 2,
                        is_left_associative: true,
                    }));
                    stream.next();
                },
                '*' => {
                    result.push(Token::Operator(Operator { 
                        kind: OperatorKind::Multiplication, 
                        precedence: 3,
                        is_left_associative: true,
                    }));
                    stream.next();
                },
                '/' => {
                    result.push(Token::Operator(Operator { 
                        kind: OperatorKind::Division, 
                        precedence: 3,
                        is_left_associative: true,
                    }));
                    stream.next();
                },
                '^' => {
                    result.push(Token::Operator(Operator { 
                        kind: OperatorKind::Power, 
                        precedence: 4,
                        is_left_associative: false,
                    }));
                    stream.next();
                },
                '(' => {
                    result.push(Token::LParen);
                    stream.next();
                }
                ')' => {
                    result.push(Token::RParen);
                    stream.next();
                }
                _ => return Err(format_err!("Unexpected token '{}'.", character))
            }
        }
    }
    return shunting_yard(result);
}

fn parse_integer(stream: &mut Peekable<Chars>) -> RustCalcResult<i64> {
    let mut accumulator: Vec<char> = Vec::new();
    loop {
        match stream.peek() {
            None => break,
            Some(&character) => match character {
                '0' ..= '9' => {
                    accumulator.push(character);
                    stream.next();
                },
                _ => break
            }
        }
    }
    let result: String = accumulator.into_iter().collect();
    let integer = result.parse::<i64>()?;
    return Ok(integer);
}

fn shunting_yard(tokens: Vec<Token>) -> RustCalcResult<Vec<Token>> {
    let mut output : VecDeque<Token> = VecDeque::new();
    let mut stack : Vec<Token> = Vec::new();
    for token in tokens {
        match token {
            Token::Integer(_) => output.push_back(token),
            Token::LParen => stack.push(token),
            Token::RParen => {
                let mut found_left = false;
                while !stack.is_empty() {
                    match stack.last() {
                        Some(Token::LParen) => { 
                            found_left = true;
                            break;
                        },
                        _ => { }
                    }
                    output.push_back(stack.pop().unwrap());
                }
                if !found_left {
                    return Err(format_err!("Missing left parenthesis in expression."));
                }
                stack.pop();
            }
            Token::Operator(op) => {
                // Rebalance the stack with consideration to operator precedence and associativeness.
                while !stack.is_empty() {
                    let item = stack.last().unwrap();
                    let cond1 = item.is_operator() && item.get_precedence() > op.precedence;
                    let cond2 = item.is_operator() && op.precedence == item.get_precedence() && item.is_left_associative();
                    if cond1 || cond2 {
                        output.push_back(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                // Push the current operator onto the stack.
                stack.push(Token::Operator(op));
            }
        }
    }

    // Pop any operations left on the stack onto the output queue.
    while !stack.is_empty() {
        output.push_back(stack.pop().unwrap());
    }

    return Ok(output.into_iter().collect());
}