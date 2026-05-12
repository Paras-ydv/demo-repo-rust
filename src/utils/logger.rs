use std::collections::HashMap;

pub struct LoggerCache<T> {
    data: HashMap<String, T>,
    max_size: usize,
}

impl<T> LoggerCache<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            data: HashMap::new(),
            max_size,
        }
    }

    pub fn insert(&mut self, key: String, value: T) {
        if self.data.len() >= self.max_size {
            if let Some(first_key) = self.data.keys().next().cloned() {
                self.data.remove(&first_key);
            }
        }
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.data.get(key)
    }
}
// auto-commit: 1778586816825