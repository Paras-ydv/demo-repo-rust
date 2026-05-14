#[cfg(test)]
mod auth_tests {
    use super::*;

    #[tokio::test]
    async fn test_auth_async() {
        let service = AuthService::new();
        service.add("item1".to_string()).await;
        service.add("item2".to_string()).await;
        let items = service.get_all().await;
        assert_eq!(items.len(), 2);
    }

    #[tokio::test]
    async fn test_auth_clear() {
        let service = AuthService::new();
        service.add("item".to_string()).await;
        service.clear().await;
        assert!(service.get_all().await.is_empty());
    }
}
// auto-commit: 1778741385570