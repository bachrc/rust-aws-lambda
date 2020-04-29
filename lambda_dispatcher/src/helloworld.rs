extern crate lambda_runtime;
extern crate log;
extern crate simple_logger;
extern crate spotipal_api;
extern crate spotipal_business;

use lambda_runtime::{error::HandlerError, lambda, Context};
use spotipal_api::helloworld::{HelloWorldRequest, HelloWorldMessage};
use spotipal_business::helloworld_service::compute_helloworld_message;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(lambda_handler);
    Ok(())
}

fn lambda_handler(e: HelloWorldRequest, _c: lambda_runtime::Context) -> Result<HelloWorldMessage, HandlerError> {
    Ok(compute_helloworld_message(e))
}