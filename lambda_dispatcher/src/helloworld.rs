#[macro_use]
extern crate lambda_runtime as lambda;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(lambda_handler);
    Ok(())
}

fn lambda_handler(
    e: LambdaRequest<HelloWorldRequest>,
    _c: lambda::Context,
) -> Result<HelloWorldMessage, lambda::HandlerError> {
    let payload: HelloWorldRequest = e.body();

    Ok(response)
}