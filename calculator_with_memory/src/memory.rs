use std::collections::HashMap;

pub struct Memory {
    pub slots: HashMap<String, f64>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Result<f64, String> {
        self.slots
            .get(key)
            .copied()
            .ok_or_else(|| format!("Key not found: {}", key))
    }

    pub fn update(&mut self, mem_name: String, value: f64) {
        self.slots
            .entry(mem_name)
            .and_modify(|v| *v += value)
            .or_insert(value);
    }
}
