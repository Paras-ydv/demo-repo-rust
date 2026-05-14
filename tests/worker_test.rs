#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_creation() {
        let service = WorkerService::new();
        assert!(service.get("key").is_none());
    }

    #[test]
    fn test_worker_set_and_get() {
        let service = WorkerService::new();
        service.set("key".to_string(), "value".to_string());
        assert_eq!(service.get("key"), Some("value".to_string()));
    }

    #[test]
    fn test_worker_remove() {
        let service = WorkerService::new();
        service.set("key".to_string(), "value".to_string());
        assert!(service.remove("key").is_some());
        assert!(service.get("key").is_none());
    }
}
// auto-commit: 1778732909647