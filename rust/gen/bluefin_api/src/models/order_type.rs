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

/// OrderType : The type of order.
/// The type of order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "MARKET")]
    Market,
    #[serde(rename = "STOP_LIMIT")]
    StopLimit,
    #[serde(rename = "STOP_MARKET")]
    StopMarket,
    #[serde(rename = "LIQUIDATION")]
    Liquidation,
    #[serde(rename = "BANKRUPTCY_LIQUIDATION")]
    BankruptcyLiquidation,
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,

}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Limit => write!(f, "LIMIT"),
            Self::Market => write!(f, "MARKET"),
            Self::StopLimit => write!(f, "STOP_LIMIT"),
            Self::StopMarket => write!(f, "STOP_MARKET"),
            Self::Liquidation => write!(f, "LIQUIDATION"),
            Self::BankruptcyLiquidation => write!(f, "BANKRUPTCY_LIQUIDATION"),
            Self::Unspecified => write!(f, "UNSPECIFIED"),
        }
    }
}

impl Default for OrderType {
    fn default() -> OrderType {
        Self::Limit
    }
}

