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

/// AdjustMarginOperation : The operation to perform on the margin
/// The operation to perform on the margin
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AdjustMarginOperation {
    #[serde(rename = "ADD")]
    Add,
    #[serde(rename = "SUBTRACT")]
    Subtract,

}

impl std::fmt::Display for AdjustMarginOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "ADD"),
            Self::Subtract => write!(f, "SUBTRACT"),
        }
    }
}

impl Default for AdjustMarginOperation {
    fn default() -> AdjustMarginOperation {
        Self::Add
    }
}

