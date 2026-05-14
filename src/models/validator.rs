use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidatorStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: ValidatorStatus,
    pub priority: u8,
}

impl Validator {
    pub fn is_active(&self) -> bool {
        self.status == ValidatorStatus::Active
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority.min(10);
    }
}
// auto-commit: 1778732482006