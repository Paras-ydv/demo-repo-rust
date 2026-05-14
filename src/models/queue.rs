use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QueueStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Queue {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: QueueStatus,
    pub priority: u8,
}

impl Queue {
    pub fn is_active(&self) -> bool {
        self.status == QueueStatus::Active
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority.min(10);
    }
}
// auto-commit: 1778735667166