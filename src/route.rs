pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: Box<dyn Fn() + Send + Sync>,
}

impl Route {
    pub fn new(method: &str, path: &str, handler: Box<dyn Fn() + Send + Sync>) -> Self {
        Route {
            method: method.to_string(),
            path: path.to_string(),
            handler,
        }
    }

    pub fn matches(&self, method: &str, path: &str) -> bool {
        self.method == method && self.path == path
    }
}
