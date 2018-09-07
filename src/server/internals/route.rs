use std::sync::Arc;

//type RouteHandler = FnMut (String) -> String;
type RouteHandler = Box<Send + Sync + 'static>;

#[derive(Clone)]
pub struct Route {
    method: String,
    path: String,
    handler: Arc<RouteHandler>
}

impl Route {
    pub fn new(method: String, path: String, handler: Arc<RouteHandler>) -> Self {
        Self {
            method,
            path,
            handler
        }
    }
}

impl Default for Route {
    fn default() -> Self {
        Self {
            method: String::from("GET"),
            path: String::from("/"),
            handler: Arc::new(Box::new(|x: String| x))
        }
    }
}
