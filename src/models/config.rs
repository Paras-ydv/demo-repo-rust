use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfigStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: ConfigStatus,
    pub priority: u8,
}

impl Config {
    pub fn is_active(&self) -> bool {
        self.status == ConfigStatus::Active
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority.min(10);
    }
}
// auto-commit: 1778736419993