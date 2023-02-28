use std::io;
use actix_web::{http, App, HttpRequest, Responder}; 
use std::cell::Cell; 
use actix_web::server;
// use rust-misc::MessageApp; 

struct State1 {
    counter: Cell<usize>,
}

struct State2 {
    counter: Cell<usize>,
}

fn index(_req:&HttpRequest<State1>) -> impl Responder {
    let count = _req.state().counter.get() + 1; 
    _req.state().counter.set(count); 
    format!("Request number in state1: {}", count);
}

fn index(_req: &HttpRequest) -> &'static str {
    "Hello sample actix web framework"
}


fn main() {
    println!("server is active");
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:3000")
        .unwrap()
        .run();
}



