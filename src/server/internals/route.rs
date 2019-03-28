use std::sync::Arc;
use std::vec::Vec;

use super::super::super::http::{
    HTTPMethod,
    request::Request,
    response::Response
};
use std::fmt::Error;

pub type RouteHandler = Box<Fn(&Request, &mut Response) -> Result<(), Error> + Send + Sync + 'static>;

#[derive(Clone)]
pub struct Route {
    method: Vec<HTTPMethod>,
    path: String,
    handler: Arc<RouteHandler>
}

impl Route {
    pub fn new(mut method: Vec<HTTPMethod>, path: String, handler: Arc<RouteHandler>) -> Self {
        method.sort();
        method.dedup();
        Self {
            method,
            path,
            handler
        }
    }

    pub fn get_method(&self) -> Vec<HTTPMethod> {
        self.method.to_vec()
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_handler(&self) -> Arc<RouteHandler> {
        Arc::clone(&self.handler)
    }

    pub fn is_route_match(&mut self, method: HTTPMethod, path: String) -> bool { // TODO: Change from bool to Result to allow for a 405 to be sent if path matches but is not a correct method
        match self.is_path_match(path) {
            true => {
                match self.method.iter().find(|&&verb| verb == method) {
                    None => false,
                    Some(_t) => true
                }
            },
            false => false
        }
    }

    #[allow(unused_variables)]
    fn is_path_match(&self, path: String) -> bool {
        let is_grabbing = false;
        let mut path_so_far = String::from("");

        for character in path.chars() {
            match char::to_string(&character).as_ref() {
                "*" => return true,
                //"{" => is_grabbing = true, // TODO: grab variables
                //"}" => is_grabbing = false,
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
            method: vec![HTTPMethod::GET, HTTPMethod::POST],
            path: String::from("/"),
            handler: Arc::new(Box::new(|req: &Request, res: &mut Response| {
                res.set_body(req.get_payload());
                res.write(true);
                Ok(())
            }))
        }
    }
}
