use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserService {
    base_url: String,
    timeout: u64,
}

impl UserService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            timeout: 30,
        }
    }

    pub async fn fetch(&self, endpoint: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = reqwest::get(&url).await?;
        Ok(response.text().await?)
    }

    pub fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }
}
// auto-commit: 1778741359434