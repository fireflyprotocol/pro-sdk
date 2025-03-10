/*
 * Bluefin API
 *
 * Bluefin API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    /// Error message
    #[serde(rename = "message")]
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Error {
        Error {
            message,
        }
    }
}

