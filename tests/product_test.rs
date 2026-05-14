#[cfg(test)]
mod product_tests {
    use super::*;

    #[tokio::test]
    async fn test_product_async() {
        let service = ProductService::new();
        service.add("item1".to_string()).await;
        service.add("item2".to_string()).await;
        let items = service.get_all().await;
        assert_eq!(items.len(), 2);
    }

    #[tokio::test]
    async fn test_product_clear() {
        let service = ProductService::new();
        service.add("item".to_string()).await;
        service.clear().await;
        assert!(service.get_all().await.is_empty());
    }
}
// auto-commit: 1778737583373