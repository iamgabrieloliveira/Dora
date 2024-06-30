use lightning_path::{Params, Router as BaseRouter, RouterMatch};
use std::str::FromStr;

pub struct Request {
    pub method: Method,
    pub path: String,
    pub params: Option<Params>,
}

impl Request {
    pub fn add_params(&mut self, params: Params) {
        self.params = Some(params);
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        let params = &self.params;

        match params {
            None => None,
            Some(p) => {
                let val = p.find(key);
                val
            }
        }
    }
}

pub struct Response {
    pub status: Status,
    pub body: String,
}

impl Response {
    pub fn ok(body: String) -> Self {
        Response {
            status: Status::Ok,
            body,
        }
    }
}

pub enum Status {
    Ok,
    NotFound,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Ok => String::from("200 OK"),
            Status::NotFound => String::from("404 Not Found"),
        }
    }
}

pub type Handler = fn(Request) -> Response;

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Option,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            "HEAD" => Ok(Method::Head),
            "OPTION" => Ok(Method::Option),
            _ => Err(()),
        }
    }
}

pub struct Route {
    pub handler: Handler,
    pub method: Method,
}

pub struct Router {
    router: BaseRouter<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            router: BaseRouter::new(),
        }
    }

    pub fn add_route(&mut self, path: &str, method: Method, handler: Handler) {
        self.router.add(path, Route { handler, method });
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.add_route(path, Method::Get, handler);
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.add_route(path, Method::Post, handler);
    }

    pub fn put(&mut self, path: &str, handler: Handler) {
        self.add_route(path, Method::Put, handler);
    }

    pub fn delete(&mut self, path: &str, handler: Handler) {
        self.add_route(path, Method::Delete, handler);
    }

    pub fn recognize(&self, path: &str) -> Result<RouterMatch<&Route>, String> {
        self.router.recognize(path)
    }
}
