use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: AuthStatus,
    pub priority: u8,
}

impl Auth {
    pub fn is_active(&self) -> bool {
        self.status == AuthStatus::Active
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority.min(10);
    }
}
// auto-commit: 1778586788843