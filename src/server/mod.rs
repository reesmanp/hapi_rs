pub mod options;
mod pool;

use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::sync::{Arc, Mutex};
use request::Request;
use response::Response;
use self::pool::{ThreadPool, ThreadPoolError};
use self::options::Options;

struct Route<'a> {
  method: String,
  path: String,
  handler: (&'a Fn(Request, Response) -> Result<String, String>)
}

pub struct Server<'a> {
  port: u32,
  host: String,
  routes: Vec<Route<'a>>,
  server_pool: ThreadPool,
  worker_pool: ThreadPool
}

impl<'a> Server<'a> {
  pub fn new(options: Options) -> Server<'a> {
    let mut server_threads: usize = options.server_threads;
    let mut worker_threads: usize = options.worker_threads;

    if server_threads == 0 {
      server_threads = 1;
    }
    if worker_threads == 0 {
      worker_threads = 8;
    }

    let server_pool = ThreadPool::new(server_threads).unwrap();
    let worker_pool = ThreadPool::new(worker_threads).unwrap();

    Server {
      port: 3000,
      host: "localhost".to_string(),
      routes: Vec::new(),
      server_pool,
      worker_pool
    }
  }

  pub fn route(&mut self, method: String, path: String, handler: (&'a Fn(Request, Response) -> Result<String, String>)) {
    let new_route = Route {
      method,
      path,
      handler
    };

    self.routes.push(new_route);
  }

  pub fn start(&self) -> Result<String, String> {
    let local_addr = format!("{}:{}", self.host, self.port);

    let server_listener = TcpListener::bind(local_addr).unwrap();
    let server_worker_listener = Arc::new(Mutex::new(server_listener.try_clone().unwrap()));

    self.server_pool.execute(move || {
      loop {
        match server_worker_listener.lock().unwrap().accept() {
          Err(_) => continue,
          Ok((_socket, addr)) => {
            //server_worker_listener.unlock();
            println!("new client: {:?}", addr);
          }
        }
      }
    });

    loop {}
/*
    thread::spawn(move || -> Result<String, String> {
      match TcpListener::bind(addr) {
        Ok(listener) => {
          for connection_attempt in listener.incoming() {
            let _conn = match connection_attempt {
              Ok(connection) => self.handle_connection(connection),
              Err(_) => continue
            };
          };
          Ok("All done!".to_string())
        },
        Err(error) => Err(error.to_string())
      }
    });
*/
    Ok("Everything is fine...".to_string())
  }

  fn handle_connection(&self, mut connection: TcpStream) {
    println!("Server is now handling the connection");
  }
}
