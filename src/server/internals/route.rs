use std::sync::Arc;
use super::thread_pool::job::FnBox;
use super::super::super::http::{
    HTTPMethod,
    request::Request,
    response::Response,
    HTTPStatusCodes
};

pub type RouteHandler = Box<Fn(&Request, &mut Response) -> String + Send + Sync + 'static>;

#[derive(Clone)]
pub struct Route {
    method: HTTPMethod,
    path: String,
    handler: Arc<RouteHandler>
}

impl Route {
    pub fn new(method: HTTPMethod, path: String, handler: Arc<RouteHandler>) -> Self {
        Self {
            method,
            path,
            handler
        }
    }

    pub fn get_method(&self) -> HTTPMethod {
        self.method
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_handler(&self) -> Arc<RouteHandler> {
        Arc::clone(&self.handler)
    }

    pub fn is_route_match(&self, method: HTTPMethod, path: String) -> bool {
        if self.method == method && self.is_path_match(path) {
            true
        } else {
            false
        }
    }

    fn is_path_match(&self, path: String) -> bool {
        let mut is_grabbing = false;
        let mut path_so_far = String::from("");

        for character in path.chars() {
            match char::to_string(&character).as_ref() {
                "*" => return true,
                "{" => is_grabbing = true, // TODO: grab variables
                "}" => is_grabbing = false,
                other => {
                    path_so_far.push_str(other);
                    if self.path.starts_with::<&str>(path_so_far.as_ref()) {
                        continue;
                    } else {
                        return false;
                    }
                }
            }
        };

        if self.path == path_so_far {
            true
        } else {
            false
        }
    }
}

impl Default for Route {
    fn default() -> Self {
        Self {
            method: HTTPMethod::GET,
            path: String::from("/"),
            handler: Arc::new(Box::new(|req: &Request, res: &mut Response|
                HTTPStatusCodes::get_generic_response(HTTPStatusCodes::c200)
            ))
        }
    }
}
