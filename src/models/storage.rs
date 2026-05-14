use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StorageStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storage {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: StorageStatus,
    pub priority: u8,
}

impl Storage {
    pub fn is_active(&self) -> bool {
        self.status == StorageStatus::Active
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority.min(10);
    }
}
// auto-commit: 1778741403015