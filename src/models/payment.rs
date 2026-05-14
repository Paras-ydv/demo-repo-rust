use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: PaymentStatus,
    pub priority: u8,
}

impl Payment {
    pub fn is_active(&self) -> bool {
        self.status == PaymentStatus::Active
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority.min(10);
    }
}
// auto-commit: 1778734489103