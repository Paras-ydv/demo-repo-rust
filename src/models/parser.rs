use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParserStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parser {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: ParserStatus,
    pub priority: u8,
}

impl Parser {
    pub fn is_active(&self) -> bool {
        self.status == ParserStatus::Active
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority.min(10);
    }
}
// auto-commit: 1778741411569