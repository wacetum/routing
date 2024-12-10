pub type Middleware = Box<dyn Fn() + Send + Sync>;

pub struct MiddlewareStack {
    middlewares: Vec<Middleware>,
}

impl MiddlewareStack {
    pub fn new() -> Self {
        MiddlewareStack {
            middlewares: Vec::new(),
        }
    }

    pub fn add(&mut self, middleware: Middleware) {
        self.middlewares.push(middleware);
    }

    pub fn run(&self) {
        for middleware in &self.middlewares {
            middleware();
        }
    }
}
