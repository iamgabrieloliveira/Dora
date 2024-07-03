> [!IMPORTANT]
> Working In Progress

# Dora - A Lightweight Web Framework for Rust

Dora is a minimalistic, web framework designed for building web applications with ease in Rust.
Its lightweight architecture allows developers to create powerful APIs and web services quickly, while maintaining simplicity and flexibility.

```rust
use dora::Dora;
use router::{Request, Response};

fn main() {
    let mut server = Dora::new();

    server.get("/", home_handler);
    server.get("/user/:id", view_user_handler);

    server.start("127.0.0.1:8000");
}

fn home_handler(request: Request) -> Response {
    Response::ok(
      String::from("Hello World")
    )
}

fn view_user_handler(request: Request) -> Response {
    let id = request.get("id").expect("Missing user id");

    Response::ok(format!("Hello User {}", id))
}

```
