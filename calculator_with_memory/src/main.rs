use calculator_with_memory::expression::eval_expression;
use calculator_with_memory::memory::Memory;
use calculator_with_memory::token::Token;
use std::io::{self, BufRead};

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
