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
    if tokens.len() != 3 {
        return Err("Invalid expression format, Expected: <lhs> <operator> <rhs>".to_string());
    }

    let lhs = eval_token(&tokens[0], memory)?;
    let rhs = eval_token(&tokens[2], memory)?;
    let result = match tokens[1] {
        Token::Plus => lhs + rhs,
        Token::Minus => lhs - rhs,
        Token::Asterisk => lhs * rhs,
        Token::Slash => {
            if rhs == 0.0 {
                return Err("Division by zero".to_string());
            }
            lhs / rhs
        }
        _ => return Err(format!("Invalid operator: {:?}", tokens[1])),
    };

    Ok(result)
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
