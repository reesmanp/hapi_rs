# hapi_rs
A rust implementation of the hapiJS framework.

### Roadmap
[Click here to see the progress to 1.0.0!](docs/Roadmap.md)

### Example
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
