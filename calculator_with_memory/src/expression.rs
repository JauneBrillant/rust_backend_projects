use crate::memory::Memory;
use crate::token::Token;

pub fn eval_expression(tokens: &[Token], memory: &Memory) -> Result<f64, String> {
    let (result, _) = eval_additive_expression(tokens, 0, memory)?;
    Ok(result)
}

fn eval_additive_expression(
    tokens: &[Token],
    index: usize,
    memory: &Memory,
) -> Result<(f64, usize), String> {
    let (mut result, mut index) = eval_multiplicative_expression(tokens, index, memory)?;

    while index < tokens.len() {
        match &tokens[index] {
            Token::Plus => {
                let (value, next) = eval_multiplicative_expression(tokens, index + 1, memory)?;
                result += value;
                index = next;
            }
            Token::Minus => {
                let (value, next) = eval_multiplicative_expression(tokens, index + 1, memory)?;
                result -= value;
                index = next;
            }
            _ => break,
        }
    }
    Ok((result, index))
}

fn eval_multiplicative_expression(
    tokens: &[Token],
    index: usize,
    memory: &Memory,
) -> Result<(f64, usize), String> {
    let (mut result, mut index) = eval_primary_expression(tokens, index, memory)?;

    while index < tokens.len() {
        match &tokens[index] {
            Token::Asterisk => {
                let (value, next) = eval_primary_expression(tokens, index + 1, memory)?;
                result *= value;
                index = next;
            }
            Token::Slash => {
                let (value, next) = eval_primary_expression(tokens, index, memory)?;
                result /= value;
                index = next;
            }
            _ => break,
        }
    }
    Ok((result, index))
}

fn eval_primary_expression(
    tokens: &[Token],
    index: usize,
    memory: &Memory,
) -> Result<(f64, usize), String> {
    let first_token = &tokens[index];
    match first_token {
        Token::LParen => {
            let (result, next) = eval_additive_expression(tokens, index + 1, memory)?;
            if next < tokens.len() && matches!(tokens[next], Token::RParen) {
                Ok((result, next + 1))
            } else {
                Err("Missing closing parenthesis".to_string())
            }
        }
        Token::Number(val) => Ok((*val, index + 1)),
        Token::MemoryRef(memory_name) => Ok((memory.get(memory_name)?, index + 1)),
        _ => unreachable!(),
    }
}
