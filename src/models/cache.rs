use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CacheStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: CacheStatus,
    pub priority: u8,
}

impl Cache {
    pub fn is_active(&self) -> bool {
        self.status == CacheStatus::Active
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority.min(10);
    }
}
// auto-commit: 1778741370550