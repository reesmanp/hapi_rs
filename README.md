# hapi_rs
A rust implementation of the hapiJs framework.

**Pre 1.0.0 | Still in active development | Backwards compatibility is not guaranteed until 1.0.0 release**

## Roadmap
[Click here to see the progress to 1.0.0!](docs/Roadmap.md)

## Why?
There is always a reason for developing something and `hapi_rs` is no different! I have spent the last few years developing with NodeJS and had a great time with it. However when projects grew, the need for a statically typed language grew too. Some of my colleagues moved towards TypeScript for their development while I found this little gem called Rust.

The server framework I have the most experience in NodeJs is hapiJs. So while I was learning rust I decided to port over hapi to rust just in case anyone followed suit from JavaScript to the Rust side of things.

## Getting Started

### Installing
This is not hosted anywhere except here, yet. Currently just clone this project in a directory and in the project using this code write this in your `.toml` file:
```toml
[dependencies]
hapi_rs = { path = "../relative/path/to/cloned/project/hapi_rs" }
```

### Examples
Any project worth their salt will have good examples of how to use their code. If mine is lacking, please let me know.

#### Basic Example
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

#### Custom Route
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
            res.flush(true);
        },
        Err(_) => {
            res.set_code(500);
            let reason_str = format!("Time before UNIX EPOCH!");
            res.set_reason(reason_str);
            res.flush(true);
        }
    }
    String::from("OK")
}
```

#### Custom Route With Generic Response And Multiple Write
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
            res.flush(false);
            
            let new_str = String::from("\nGet hapi now!");
            res.set_body(new_str);
            res.flush(true);
            
            let another_new_str = String::from("\nhappy*, oops!");
            res.set_body(another_new_str);
            res.flush(true);
        },
        Err(_) => {
            res.set_code(HTTPStatusCodes::c500);
            let reason_str = HTTPStatusCodes::get_generic_reason(HTTPStatusCodes::c500);
            res.set_reason(reason_str);
            res.flush(true);
        }
    }
    String::from("OK")
}
```

## Versioning
This project uses [semantic](https://semver.org/) versioning.

## Contributing
If you would like to contribute, just let me know. I do not have a contributing documentation quite yet.
