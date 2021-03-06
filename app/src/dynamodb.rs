use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/api.js")]
extern "C" {
    async fn dynamodb_request() -> JsValue;
}

struct DynamoDBRequest {
    profile: String,
    region: String,
    table_name: String,
}

impl DynamoDBRequest {}

pub async fn request() -> String {
    let result = dynamodb_request().await;
    result.as_string().unwrap()
}
