use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct SchedulerService {
    cache: Arc<Mutex<HashMap<String, String>>>,
}

impl SchedulerService {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.cache.lock().unwrap().get(key).cloned()
    }

    pub fn set(&self, key: String, value: String) {
        self.cache.lock().unwrap().insert(key, value);
    }

    pub fn remove(&self, key: &str) -> Option<String> {
        self.cache.lock().unwrap().remove(key)
    }
}
// auto-commit: 1778737926924