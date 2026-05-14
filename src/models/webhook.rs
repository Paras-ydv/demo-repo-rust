use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WebhookStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: WebhookStatus,
    pub priority: u8,
}

impl Webhook {
    pub fn is_active(&self) -> bool {
        self.status == WebhookStatus::Active
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority.min(10);
    }
}
// auto-commit: 1778732745170