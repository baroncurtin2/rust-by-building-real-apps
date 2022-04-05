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
use server::Server;
use http::Request;
use http::Method;

mod server;
mod http;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/