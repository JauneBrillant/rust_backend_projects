use std::collections::HashMap;

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

    fn add(&mut self, mem_name: String, prev_result: f64) {
        let entry = self.slots.entry(mem_name).or_insert(0.0);
        *entry += prev_result;
    }
}

fn main() {
    let mut memory = Memory::new();
    let mut prev_result = 0.0;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens[0].starts_with("mem") && tokens[0].ends_with("+") {
            let mem_name = tokens[0][3..tokens[0].len() - 1].to_string();
            memory.add(mem_name.clone(), prev_result);
            println!(" => {}", memory.get(&mem_name).unwrap());
            continue;
        } else if tokens[0].starts_with("mem") && tokens[0].ends_with("-") {
            let mem_name = tokens[0][3..tokens[0].len() - 1].to_string();
            memory.add(mem_name.clone(), -prev_result);
            println!(" => {}", memory.get(&mem_name).unwrap());
            continue;
        }

        let operator = tokens[1];
        let eval_token = |token: &str| -> f64 {
            if token.starts_with("mem") {
                memory.get(&token[3..]).unwrap()
            } else {
                token.parse().unwrap()
            }
        };
        let lhs = eval_token(tokens[0]);
        let rhs = eval_token(tokens[2]);

        let result = match operator {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => lhs / rhs,
            _ => unreachable!(),
        };

        println!(" => {}", result);

        prev_result = result;
    }

    println!("program terminated")
}
