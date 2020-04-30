#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate spotipal_business;
extern crate spotipal_api;

use rocket_contrib::json::Json;

use spotipal_api::helloworld::{HelloWorldMessage, HelloWorldRequest};
use spotipal_business::helloworld_service::compute_helloworld_message;

#[post("/hello-world", data = "<request>")]
fn helloworld(request: Json<HelloWorldRequest>) -> Json<HelloWorldMessage> {
    Json(compute_helloworld_message(&request.0))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![helloworld])
        .launch();
}