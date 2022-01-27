#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

use rocket::{Build, Request, Rocket};
use rocket::response::{content, status};
use rocket::http::Status;

#[get("/")]
fn indexFunction() -> &'static str {
    "Hello, From the web app"
}

#[get("/helloworld")]
fn customREST() -> &'static str {
    "this message is for the customized string"
}

#[launch]
fn rocketFunction() -> Rocket<Build> {
    rocket::build()
    .mount("/", routes![indexFunction])
    .mount("/helloworld", routes![customREST])
}
