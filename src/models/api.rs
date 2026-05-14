use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApiStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Api {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: ApiStatus,
    pub priority: u8,
}

impl Api {
    pub fn is_active(&self) -> bool {
        self.status == ApiStatus::Active
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority.min(10);
    }
}
// auto-commit: 1778737419271