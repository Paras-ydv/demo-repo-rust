use std::fmt;

#[derive(Debug, Clone)]
pub struct Cache {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

impl Cache {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: None,
        }
    }

    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }
}

impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}
// auto-commit: 1778737919282