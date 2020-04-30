extern crate lambda_runtime;
extern crate log;
extern crate simple_logger;
extern crate spotipal_api;
extern crate spotipal_business;

mod lambda_gateway;

use lambda_runtime::{Context, error::HandlerError, lambda};

use spotipal_api::helloworld::HelloWorldRequest;
use spotipal_business::helloworld_service::compute_helloworld_message;
use lambda_gateway::{LambdaRequest, LambdaResponse, LambdaResponseBuilder};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(lambda_handler);
    Ok(())
}

fn lambda_handler(e: LambdaRequest<HelloWorldRequest>, _c: Context) -> Result<LambdaResponse, HandlerError> {
    let request = e.body();

    let response = LambdaResponseBuilder::new()
        .with_status(200)
        .with_json(compute_helloworld_message(request))
        .build();

    Ok(response)
}