use crate::router::{Handler, Method, Request, Response, Router, Status};
use std::str::FromStr;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub struct Dora {
    router: Router,
}

impl Dora {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.router.get(path, handler);
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.router.post(path, handler);
    }

    pub fn put(&mut self, path: &str, handler: Handler) {
        self.router.put(path, handler);
    }

    pub fn delete(&mut self, path: &str, handler: Handler) {
        self.router.delete(path, handler);
    }

    pub fn start(&self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();

        for stream in listener.incoming() {
            self.handle_connection(stream.unwrap());
        }
    }

    pub fn handle_connection(&self, stream: TcpStream) {
        let mut req = self.parse_request(&stream);

        let m = self.router.recognize(req.path.as_str());

        match m {
            Ok(m) => {
                req.add_params(m.params);
                let response = (m.handler.handler)(req);
                self.respond(&stream, response);
            }
            Err(_) => {
                let response = self.not_found_handler();
                self.respond(&stream, response);
            }
        }
    }

    pub fn parse_request(&self, stream: &TcpStream) -> Request {
        let mut reader = BufReader::new(stream);
        let mut buffer = String::new();

        reader.read_line(&mut buffer).unwrap();

        let mut parts = buffer.split_whitespace();

        let method = parts.next().unwrap();
        let path = parts.next().unwrap();

        Request {
            method: Method::from_str(method).expect("Invalid HTTP method"),
            path: path.to_string(),
            params: None,
        }
    }

    pub fn respond(&self, mut stream: &TcpStream, response: Response) {
        let response = format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
            response.status.to_string(),
            response.body.len(),
            response.body
        );

        stream.write(response.as_bytes()).unwrap();
    }

    pub fn not_found_handler(&self) -> Response {
        Response {
            status: Status::NotFound,
            body: "<h1>Not Found</h1>".to_string(),
        }
    }
}
