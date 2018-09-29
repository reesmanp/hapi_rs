pub mod internals;

use self::internals::{
    options::ServerOptions,
    route::Route,
    thread_pool::ThreadPool,
    thread_pool::job::FnBox
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
use std::marker::Sync;

pub struct Server {
    port: u32,
    host: String,
    routes: Vec<Route>,
    server_thread_pool: ThreadPool,
    worker_thread_pool: ThreadPool
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
            worker_thread_pool: ThreadPool::new(options.get_worker_threads())
        }
    }

    pub fn route(&mut self, new_route: Route) {
        // TODO: Add more route validation here
        self.routes.push(new_route);
    }

    pub fn start(self) -> Result<String> {
        let host_url = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(host_url).unwrap();
        let worker_listener = Arc::new(Mutex::new(listener.try_clone().unwrap()));
        let mut shared_routes = Arc::new(self.routes.to_vec());
        let worker_thread_pool = Arc::new(self.worker_thread_pool);

        // Begin Accepting Connections on all Server Threads
        let server_thread_job = move || {
            loop {
                match worker_listener.lock().unwrap().accept() {
                    Err(_) => continue,
                    Ok((stream, _addr)) => handle_connection(stream, &mut shared_routes, Arc::clone(&worker_thread_pool))
                }
            }
        };

        self.server_thread_pool.execute_job(server_thread_job);

        loop {}

        Ok(String::from("OK"))
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            port: 3000,
            host: String::from("localhost"),
            routes: vec![],
            server_thread_pool: ThreadPool::new(1),
            worker_thread_pool: ThreadPool::new(2)
        }
    }
}

fn handle_connection(mut stream: TcpStream, routes: &mut Arc<Vec<Route>>, pool: Arc<ThreadPool>) {
    let mut buffer:[u8; 512] = [0; 512];
    stream.read(&mut buffer).unwrap();
    let (_, buffer_split) = buffer.split_at(0);

    let request = match buffer_split.len() {
        0 => {
            // Request has no content
            // TODO: Allow user to override generic response with custom route
            stream.write(HTTP::get_generic_response_string(HTTPStatusCodes::c400, HTTPVersion::HTTP20).as_ref()).unwrap();
            stream.flush().unwrap();
            None
        },
        _ => {
            Request::parse_request(buffer_split)
        }
    };

    // Get the String response to write to the stream
    match request {
        None => {
            // Request didn't parse correctly
            // Bad Request
            // TODO: Allow user to override generic response with custom route
            stream.write(HTTP::get_generic_response_string(HTTPStatusCodes::c400, HTTPVersion::HTTP20).as_ref()).unwrap();
            stream.flush().unwrap();
        },
        Some(some_request) => {
            let mut route_response = String::from("");

            println!("{:?}", some_request.get_method());
            println!("{:?}", some_request.get_path());
            println!("{:?}", some_request.get_version());
            println!("{}", some_request.get_headers().get_headers_formatted());
            println!("{}", some_request.get_payload());

            // Valid request
            // Searching for matching route in order it was added
            for route in routes.iter() {
                match route.is_route_match(some_request.get_method(), some_request.get_path()) {
                    true => {
                        // Route exists
                        // Call route handler
                        let handler_box = route.get_handler();
                        //let handler_box = Box::new(**route.get_handler());
                        pool.execute_handler(handler_box, some_request, Response::from_stream(stream));
                        //route_response = (**route.get_handler())(&some_request, &mut Response::default());
                        return;
                    },
                    false => continue
                }
            }

            // Route was not found
            // Send 404
            // TODO: Allow user to override generic response with custom route
            stream.write(HTTP::get_generic_response_string(HTTPStatusCodes::c404, HTTPVersion::HTTP20).as_ref()).unwrap();
            stream.flush().unwrap();
        }
    };
}
