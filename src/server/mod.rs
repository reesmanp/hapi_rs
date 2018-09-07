pub mod internals;

use self::internals::{
    options::ServerOptions,
    route::Route,
    thread_pool::ThreadPool
};
use super::http::{HTTP, HTTPStatusCodes};
use super::http::request::Request;
use std::vec::Vec;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::io::*;

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

    pub fn start(&self) -> Result<String> {
        let host_url = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(host_url).unwrap();
        let worker_listener = Arc::new(Mutex::new(listener.try_clone().unwrap()));
        let shared_routes = Arc::new(self.routes.to_vec());

        // Begin Accepting Connections on all Server Threads
        self.server_thread_pool.execute_all_continuous(move || {
            loop {
                match worker_listener.lock().unwrap().accept() {
                    Err(_) => continue,
                    Ok((stream, _addr)) => handle_connection(stream, &shared_routes)
                }
            }
        });

        loop {}

        Ok(String::from("OK"))
    }

    //fn handle_connection(&self, stream: TcpStream) {}
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

fn handle_connection(mut stream: TcpStream, routes: &Arc<Vec<Route>>) {
    let mut buffer:[u8; 512] = [0; 512];
    stream.read(&mut buffer).unwrap();
    let (_, buffer_split) = buffer.split_at(0);

    let request = match buffer_split.len() {
        0 => {
            stream.write(HTTPStatusCodes::get_generic_response(HTTPStatusCodes::c400).as_ref()).unwrap();
            stream.flush().unwrap();
            None
        },
        _ => {
            Request::parse_request(buffer_split)
        }
    };

    let response = match request {
        None => {
            HTTPStatusCodes::get_generic_response(HTTPStatusCodes::c400)
        },
        Some(some_request) => {
            println!("{:?}", some_request.get_method());
            println!("{:?}", some_request.get_path());
            println!("{:?}", some_request.get_version());
            println!("{}", some_request.get_headers().get_headers_formatted());
            HTTPStatusCodes::get_generic_response(HTTPStatusCodes::c200)
        }
    };

    stream.write(response.as_ref()).unwrap();
    stream.flush().unwrap();
    println!("Response: {}", response);
}
