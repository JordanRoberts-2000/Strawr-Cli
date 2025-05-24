use std::any::type_name;

use reqwest::blocking::Response;
use serde::de::DeserializeOwned;

use crate::error::ParseError;

pub fn parse_response(res: Response) -> Result<serde_json::Value, ParseError> {
    let response_url = res.url().clone();
    res.json::<serde_json::Value>()
        .map_err(|e| ParseError::JsonResponse {
            source: e,
            url: response_url,
        })
}

pub fn parse_model<T: DeserializeOwned>(json: &serde_json::Value) -> Result<T, ParseError> {
    serde_json::from_value(json.clone()).map_err(|e| ParseError::JsonModel {
        source: e,
        json: json.to_string(),
        model: type_name::<T>().to_string(),
    })
}
