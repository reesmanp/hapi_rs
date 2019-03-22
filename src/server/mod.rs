pub mod internals;

use self::internals::{
    options::ServerOptions,
    route::Route,
    thread_pool::ThreadPool
};
use super::http::HTTPStatusCodes;
use super::http::HTTPVersion;
use super::http::request::Request;
use super::http::response::Response;
use super::http::HTTP;
use std::vec::Vec;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::io::*;


pub struct Server {
    port: u32,
    host: String,
    routes: Vec<Route>,
    server_thread_pool: ThreadPool,
    worker_thread_pool: ThreadPool,
    default_http_version: HTTPVersion
}

impl Server {
    pub fn new(options: &ServerOptions) -> Self {
        assert!(options.get_server_threads() > 0);
        assert!(options.get_worker_threads() > 0);

        Self {
            port: options.get_port(),
            host: options.get_host(),
            routes: vec![],
            server_thread_pool: ThreadPool::new(options.get_server_threads()),
            worker_thread_pool: ThreadPool::new(options.get_worker_threads()),
            default_http_version: options.get_default_http_version()
        }
    }

    pub fn route(&mut self, new_route: Route) {
        // TODO: Add more route validation here
        self.routes.push(new_route);
    }

    pub fn start(self) -> ! {
        let host_url = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(host_url).unwrap();
        let worker_listener = Arc::new(Mutex::new(listener.try_clone().unwrap()));
        let mut shared_routes = Arc::new(self.routes.to_vec());
        let worker_thread_pool = Arc::new(self.worker_thread_pool);
        let http_version = self.default_http_version.clone();

        // Begin Accepting Connections on all Server Threads
        let server_thread_job = move || {
            loop {
                match worker_listener.lock().unwrap().accept() {
                    Err(_) => continue,
                    Ok((stream, _addr)) => handle_connection(stream, &mut shared_routes, Arc::clone(&worker_thread_pool), http_version)
                }
            }
        };

        self.server_thread_pool.execute_job(server_thread_job);

        loop {}

        //Ok(String::from("OK"))
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            port: 3000,
            host: String::from("localhost"),
            routes: vec![],
            server_thread_pool: ThreadPool::new(1),
            worker_thread_pool: ThreadPool::new(2),
            default_http_version: HTTPVersion::HTTP20
        }
    }
}

fn handle_connection(mut stream: TcpStream, routes: &mut Arc<Vec<Route>>, pool: Arc<ThreadPool>, http_version: HTTPVersion) {
    let mut buffer:[u8; 512] = [0; 512];
    stream.read(&mut buffer).unwrap();

    let request = match buffer.len() != 0 {
        false => {
            // Request has no content
            // TODO: Allow user to override generic response with custom route
            stream.write(HTTP::get_generic_response_string(HTTPStatusCodes::BadRequest400, http_version).as_ref()).unwrap();
            stream.flush().unwrap();
            None
        },
        true => {
            Request::parse_request(&buffer)
        }
    };

    // Get the String response to write to the stream
    match request {
        None => {
            // Request didn't parse correctly
            // Bad Request
            // TODO: Allow user to override generic response with custom route
            stream.write(HTTP::get_generic_response_string(HTTPStatusCodes::BadRequest400, http_version).as_ref()).unwrap();
            stream.flush().unwrap();
        },
        Some(some_request) => {
            println!("{:?}", some_request.get_method());
            println!("{:?}", some_request.get_path());
            println!("{:?}", some_request.get_version());
            println!("{}", some_request.get_headers().get_headers_formatted());
            println!("{}", some_request.get_payload());

            // Valid request
            // Searching for matching route in order it was added
            for route in routes.to_vec().iter_mut() {
                match route.is_route_match(some_request.get_method(), some_request.get_path()) {
                    true => {
                        // Route exists
                        // Call route handler
                        let handler_box = route.get_handler();
                        let mut response = Response::from_stream(stream);
                        response.set_version(http_version);
                        pool.execute_handler(handler_box, some_request, response);
                        return;
                    },
                    false => continue
                }
            }

            // Route was not found
            // Send 404
            // TODO: Allow user to override generic response with custom route
            let not_found_response = HTTP::get_generic_response_string(HTTPStatusCodes::NotFound404, some_request.get_version());
            println!("404\n{}", not_found_response);
            stream.write(not_found_response.as_ref()).unwrap();
            stream.flush().unwrap();
        }
    };
}
