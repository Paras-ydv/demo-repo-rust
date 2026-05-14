use tokio::sync::RwLock;
use std::sync::Arc;

pub struct QueueService {
    data: Arc<RwLock<Vec<String>>>,
}

impl QueueService {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add(&self, item: String) {
        let mut data = self.data.write().await;
        data.push(item);
    }

    pub async fn get_all(&self) -> Vec<String> {
        self.data.read().await.clone()
    }

    pub async fn clear(&self) {
        self.data.write().await.clear();
    }
}
// auto-commit: 1778741378813