mod dora;
mod router;

use dora::Dora;
use router::{Request, Response};

fn main() {
    let mut server = Dora::new();

    server.get("/", home_handler);
    server.get("/user/:id", view_user_handler);

    server.start("127.0.0.1:8000");
}

fn home_handler(_request: Request) -> Response {
    Response::ok(String::from("GET"))
}

fn view_user_handler(request: Request) -> Response {
    let id = request.get("id").expect("Missing user id");

    Response::ok(format!("Hello User {}", id))
}
