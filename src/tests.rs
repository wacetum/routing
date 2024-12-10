#[cfg(test)]
mod tests {
    use crate::{Route, Router};

    fn test_handler() {
        println!("Handler executed");
    }

    #[test]
    fn test_route_matching() {
        let route = Route::new("GET", "/test", Box::new(test_handler));
        assert!(route.matches("GET", "/test"));
        assert!(!route.matches("POST", "/test"));
    }

    #[test]
    fn test_router_resolve() {
        let mut router = Router::new();
        router.add_route(Route::new("GET", "/test", Box::new(test_handler)));

        let resolved = router.resolve("GET", "/test");
        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap().path, "/test");
    }
}
