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
use hapi_rs::http::HTTPVersion;

fn main() {
    let options = ServerOptions::new(
        String::from("localhost"),
        3000,
        4,
        4,
        HTTPVersion::HTTP11
    );

    let mut server = Server::new(&options);

    // Add routes here
    server.route(Route::default());

    server.start();
}
```

#### Custom Route
```rust
// ...

use hapi_rs::http::{
    HTTPMethod,
    HTTPVersion,
    Request,
    Response
};

use std::time::SystemTime;
use std::sync::Arc;

fn main() {
    let options = ServerOptions::new(String::from("localhost"), 3000, 4, 4, HTTPVersion::HTTP11);

    let mut server = Server::new(&options);

    // Add routes here
    server.route(Route::default());
    server.route(Route::new(
        vec![HTTPMethod::GET],
        String::from("/test"),
        Arc::new(Box::new(|req: &Request, res: &mut Response| my_func(req, res)))
    ));

    server.start();
}

fn my_func(req: &Request, res: &mut Response) -> Result<(), Error> {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            let secs = n.as_secs();
            let time_str = format!("Time is: {}", secs);
            res.set_body(time_str);
            res.write(false);
        },
        Err(_) => {
            res.set_code(500);
            let reason_str = format!("Time before UNIX EPOCH!");
            res.set_reason(reason_str);
            res.write(false);
        }
    }
    Ok(())
}
```

#### Custom Route With Generic Response And Multiple Write
```rust
// ...

use hapi_rs::http::{
    HTTPVersion,
    HTTPStatusCodes,
    Request,
    Response
};

use std::time::SystemTime;
use std::sync::Arc;

// ...

fn my_func(req: &Request, res: &mut Response) -> Result<(), Error> {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            let secs = n.as_secs();
            let time_str = format!("Time is: {}", secs);
            res.set_body(time_str);
            res.write(false);
            
            let new_str = String::from("\nTime to get hapi now!");
            res.set_body(new_str);
            res.write(true);
            
            let another_new_str = String::from("\nhappy*, oops!");
            res.set_body(another_new_str);
            res.write(true);
        },
        Err(_) => {
            res.set_default_code_and_reason(HTTPStatusCodes::c500);
            res.write(true);
        }
    }
    Ok(())
}
```

## Versioning
This project uses [semantic](https://semver.org/) versioning.

## Contributing
If you would like to contribute, just let me know. I do not have a contributing documentation quite yet.
