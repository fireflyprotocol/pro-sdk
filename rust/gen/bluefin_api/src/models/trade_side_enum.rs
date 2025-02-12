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

/// TradeSideEnum : Trade side based on the user order in this trade.
/// Trade side based on the user order in this trade.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TradeSideEnum {
    #[serde(rename = "LONG")]
    Long,
    #[serde(rename = "SHORT")]
    Short,

}

impl std::fmt::Display for TradeSideEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Long => write!(f, "LONG"),
            Self::Short => write!(f, "SHORT"),
        }
    }
}

impl Default for TradeSideEnum {
    fn default() -> TradeSideEnum {
        Self::Long
    }
}

