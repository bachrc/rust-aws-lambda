use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HelloWorldRequest {
    pub name: String
}

#[derive(Deserialize, Serialize)]
pub struct HelloWorldMessage {
    pub message: String
}