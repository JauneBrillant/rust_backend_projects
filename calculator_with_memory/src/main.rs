use calculator_with_memory::memory::Memory;
use calculator_with_memory::token::Token;
use std::io::{self, BufRead};

fn eval_token(token: &Token, memory: &Memory) -> Result<f64, String> {
    match token {
        Token::Number(val) => Ok(*val),
        Token::MemoryRef(name) => memory.get(name),
        _ => Err("Failed eval_token".to_string()),
    }
}

fn eval_expression(tokens: &[Token], memory: &Memory) -> Result<f64, String> {
    eval_additive_expression(tokens, memory)
}

fn eval_additive_expression(tokens: &[Token], memory: &Memory) -> Result<f64, String> {
    let (mut result, mut index) = eval_multiplicative_expression(tokens, 0, memory);

    while index < tokens.len() {
        match &tokens[index] {
            Token::Plus => {
                let (value, next) = eval_multiplicative_expression(tokens, index + 1, memory);
                result += value;
                index = next;
            }
            Token::Minus => {
                let (value, next) = eval_multiplicative_expression(tokens, index + 1, memory);
                result -= value;
                index = next;
            }
            _ => break,
        }
    }
    Ok(result)
}

fn eval_multiplicative_expression(tokens: &[Token], index: usize, memory: &Memory) -> (f64, usize) {
    let mut index = index;
    let mut result = match eval_token(&tokens[index], memory) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {}", e);
            return (0.0, index);
        }
    };

    index += 1;
    while index < tokens.len() {
        match &tokens[index] {
            Token::Asterisk => {
                result *= eval_token(&tokens[index + 1], memory).unwrap();
                index += 2;
            }
            Token::Slash => {
                result /= eval_token(&tokens[index + 1], memory).unwrap();
                index += 2;
            }
            _ => break,
        }
    }
    (result, index)
}

fn main() {
    let mut memory = Memory::new();
    let mut prev_result = 0.0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        let tokens: Vec<Token> = match line
            .split_whitespace()
            .map(|word| Token::parse(word, &memory.slots))
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(toks) => toks,
            Err(e) => {
                eprintln!("Error parsing tokens: {}", e);
                break;
            }
        };

        if tokens.len() == 1 {
            match &tokens[0] {
                Token::MemoryPlus(name) => {
                    memory.update(name.clone(), prev_result);
                    println!(" => {:?}", memory.get(name).unwrap());
                }
                Token::MemoryMinus(name) => {
                    memory.update(name.clone(), -prev_result);
                    println!(" => {:?}", memory.get(name).unwrap());
                }
                _ => unreachable!(),
            }
            continue;
        }

        match eval_expression(&tokens, &memory) {
            Ok(result) => {
                println!(" => {}", result);
                prev_result = result
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    println!("Program terminated.")
}
