# hapi_rs
A rust implementation of the hapiJS framework.

### Roadmap
[Click here to see the progress to 1.0.0!](docs/Roadmap.md)

## Examples
### Basic Example
```rust
extern crate hapi_rs;

use hapi_rs::server::Server;
use hapi_rs::server::internals::options::ServerOptions;
use hapi_rs::server::internals::route::Route;

fn main() {
    let options = ServerOptions::new(
        String::from("localhost"),
        3000,
        4,
        4
    );

    let mut server = Server::new(&options);

    // Add routes here
    server.route(Route::default());

    match server.start() {
        Ok(t) => println!("OK: {}", t),
        Err(e) => println!("Error: {}", e)
    }
}
```

### Custom Route
```rust
...

use hapi_rs::http::{
    HTTPMethod,
    request::Request,
    response::Response
};

use std::time::SystemTime;
use std::sync::Arc;

fn main() {
    let options = ServerOptions::new(String::from("localhost"), 3000, 4, 4);

    let mut server = Server::new(&options);

    // Add routes here
    server.route(Route::default());
    server.route(Route::new(
        vec![HTTPMethod::GET],
        String::from("/test"),
        Arc::new(Box::new(|req: &Request, res: &mut Response| my_func(req, res)))
    ));

    match server.start() {
        Ok(t) => println!("OK: {}", t),
        Err(e) => println!("Error: {}", e)
    }
}

fn my_func(req: &Request, res: &mut Response) -> String {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            let secs = n.as_secs();
            let time_str = format!("Time is: {}", secs);
            res.set_body(time_str);
            res.flush();
        },
        Err(_) => {
            res.set_code(500);
            let reason_str = format!("Time before UNIX EPOCH!");
            res.set_reason(reason_str);
            res.flush();
        }
    }
    String::from("OK")
}
```

### Custom Route with generic response
```rust
...

use hapi_rs::http::{
    HTTPStatusCodes,
    request::Request,
    response::Response
};

use std::time::SystemTime;
use std::sync::Arc;

...

fn my_func(req: &Request, res: &mut Response) -> String {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            let secs = n.as_secs();
            let time_str = format!("Time is: {}", secs);
            res.set_body(time_str);
            res.flush();
        },
        Err(_) => {
            res.set_code(HTTPStatusCodes::c500);
            let reason_str = HTTPStatusCodes::get_generic_reason(HTTPStatusCodes::c500);
            res.set_reason(reason_str);
            res.flush();
        }
    }
    String::from("OK")
}
```
