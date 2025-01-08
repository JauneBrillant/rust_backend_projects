use std::collections::HashMap;

#[derive(Debug)]
pub enum Token {
    Number(f64),
    MemoryRef(String),
    MemoryPlus(String),
    MemoryMinus(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
}

impl Token {
    pub fn parse(input: &str, memory: &HashMap<String, f64>) -> Result<Token, String> {
        if let Ok(number) = input.parse::<f64>() {
            return Ok(Token::Number(number));
        }

        if input.starts_with("mem") {
            let name = input[3..input.len() - 1].trim().to_string();
            if input.ends_with("+") {
                return Ok(Token::MemoryPlus(name));
            } else if input.ends_with("-") {
                return Ok(Token::MemoryMinus(name));
            }
        }

        if memory.contains_key(input) {
            return Ok(Token::MemoryRef(input.to_string()));
        }

        match input {
            "+" => Ok(Token::Plus),
            "-" => Ok(Token::Minus),
            "*" => Ok(Token::Asterisk),
            "/" => Ok(Token::Slash),
            _ => Err(format!("Unknow token: {}", input)),
        }
    }
}
