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

/// TradeType : The type of trade.
/// The type of trade.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TradeType {
    #[serde(rename = "ORDER")]
    Order,
    #[serde(rename = "LIQUIDATION")]
    Liquidation,
    #[serde(rename = "DELEVERAGE")]
    Deleverage,
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,

}

impl std::fmt::Display for TradeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Order => write!(f, "ORDER"),
            Self::Liquidation => write!(f, "LIQUIDATION"),
            Self::Deleverage => write!(f, "DELEVERAGE"),
            Self::Unspecified => write!(f, "UNSPECIFIED"),
        }
    }
}

impl Default for TradeType {
    fn default() -> TradeType {
        Self::Order
    }
}

