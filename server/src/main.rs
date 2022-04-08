/*
Strings
- &str is a string slice to an immutable reference to a part of a String
- Slicing strings with integers is not preferred
- Rust strings are stored in UTF-8

Modules
- All modules are private and all members are private

Special Types
- Option<T> is a type that can be either Some(T) or None
- Result<T, E> is a type that can be either Ok(T) or Err(E)
*/
#![allow(dead_code)]

use std::env;
use website_handler::WebsiteHandler;
use crate::server::Server;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Public path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
