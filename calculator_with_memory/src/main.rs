use std::collections::HashMap;
use std::io::{self, BufRead};

struct Memory {
    slots: HashMap<String, f64>,
}

impl Memory {
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Option<f64> {
        self.slots.get(key).copied()
    }

    fn update(&mut self, mem_name: String, value: f64) {
        self.slots
            .entry(mem_name)
            .and_modify(|v| *v += value)
            .or_insert(value);
    }
}

// or_else
// None, Errの時に代替の処理を行いたい時に使う

// ok_or_else
// OptionをResultに変換するときに、Noneに対応するエラー値を生成したい時に使う
fn eval_token(token: &str, memory: &Memory) -> Result<f64, String> {
    memory
        .get(token)
        .or_else(|| token.parse().ok())
        .ok_or_else(|| format!("Failed to parse token: {}", token))
}

fn eval_expression(tokens: &[&str], memory: &Memory) -> Result<f64, String> {
    if tokens.len() != 3 {
        return Err("Invalid expression format, Expected: <lhs> <operator> <rhs>".to_string());
    }

    let lhs = eval_token(tokens[0], memory)?;
    let rhs = eval_token(tokens[2], memory)?;
    let result = match tokens[1] {
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "*" => lhs * rhs,
        "/" => {
            if rhs == 0.0 {
                return Err("Division by zero".to_string());
            }
            lhs / rhs
        }
        _ => return Err(format!("Invalid operator: {}", tokens[1])),
    };

    Ok(result)
}

fn process_memory_command(line: &str, prev_result: f64, memory: &mut Memory) -> Result<(), String> {
    let is_addition = line.ends_with("+");
    let is_subtraction = line.ends_with("-");
    let memory_name = line[3..line.len() - 1].trim().to_string();

    if is_addition || is_subtraction {
        let val = if is_addition {
            prev_result
        } else {
            -prev_result
        };
        memory.update(memory_name.clone(), val);
        println!(" => {}", memory.get(&memory_name).unwrap_or(0.0));
        Ok(())
    } else {
        Err("Invalid memory operation. Use 'mem<name>+' or 'mem<name>-'.".to_string())
    }
}

fn main() {
    let mut memory = Memory::new();
    let mut prev_result = 0.0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        if line.starts_with("mem") {
            if let Err(e) = process_memory_command(&line, prev_result, &mut memory) {
                eprintln!("Error: {}", e);
            }
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
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
