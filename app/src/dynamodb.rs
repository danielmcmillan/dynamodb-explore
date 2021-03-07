use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/api.js")]
extern "C" {
    async fn dynamodb_request(
        profile: &str,
        region: &str,
        table_name: &str,
        expression_attribute_names: JsValue, // Option<HashMap<&str, &str>>
        expression_attribute_values: JsValue, // Option<HashMap<&str, &str>>
        query_input: JsValue,
    ) -> JsValue; // ResultData
}

pub type ResultData = Vec<HashMap<String, String>>;

pub struct Input {
    pub profile: String,
    pub region: String,
    pub table_name: String,
    pub request_type: RequestType,
    pub expression_attribute_names: Option<HashMap<String, String>>,
    pub expression_attribute_values: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct QueryInput {
    pub key_condition_expression: String,
}

pub enum RequestType {
    Scan,
    Query(QueryInput),
    Update,
    Put,
    Delete,
}

#[derive(Debug, Clone)]
pub struct DynamoDBRequestFailed;

impl fmt::Display for DynamoDBRequestFailed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "An error ocurred while performing the request to DynamoDB"
        )
    }
}

impl Error for DynamoDBRequestFailed {}

pub async fn request(input: &Input) -> Result<ResultData, DynamoDBRequestFailed> {
    let result = dynamodb_request(
        input.profile.as_str(),
        input.region.as_str(),
        input.table_name.as_str(),
        JsValue::from_serde(&input.expression_attribute_names).unwrap_or(JsValue::UNDEFINED),
        JsValue::from_serde(&input.expression_attribute_values).unwrap_or(JsValue::UNDEFINED),
        match &input.request_type {
            RequestType::Scan => JsValue::UNDEFINED,
            RequestType::Query(query) => JsValue::from_serde(&query).unwrap_or(JsValue::UNDEFINED),
            _ => JsValue::UNDEFINED,
        },
    )
    .await;
    let data: Option<ResultData> = result.into_serde().ok();
    match data {
        Some(data) => Ok(data),
        None => Err(DynamoDBRequestFailed),
    }
}
