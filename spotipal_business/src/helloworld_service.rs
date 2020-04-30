extern crate spotipal_api;

use spotipal_api::helloworld::HelloWorldMessage;
use spotipal_api::helloworld::HelloWorldRequest;

pub fn compute_helloworld_message(request: &HelloWorldRequest) -> HelloWorldMessage {
    HelloWorldMessage {
        message : format!("Hello {} ! And welcome on my app.", request.name)
    }
}