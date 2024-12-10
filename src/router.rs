use crate::route::Route;

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Router { routes: Vec::new() }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn resolve(&self, method: &str, path: &str) -> Option<&Route> {
        self.routes.iter().find(|route| route.matches(method, path))
    }
}
