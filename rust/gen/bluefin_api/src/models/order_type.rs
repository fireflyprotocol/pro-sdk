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

/// OrderType : The type of order. (BANKRUPTCY_LIQUIDATION is deprecated)
/// The type of order. (BANKRUPTCY_LIQUIDATION is deprecated)
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
    #[serde(rename = "STOP_LOSS_MARKET")]
    StopLossMarket,
    #[serde(rename = "TAKE_PROFIT_MARKET")]
    TakeProfitMarket,
    #[serde(rename = "STOP_LOSS_LIMIT")]
    StopLossLimit,
    #[serde(rename = "TAKE_PROFIT_LIMIT")]
    TakeProfitLimit,
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
            Self::StopLossMarket => write!(f, "STOP_LOSS_MARKET"),
            Self::TakeProfitMarket => write!(f, "TAKE_PROFIT_MARKET"),
            Self::StopLossLimit => write!(f, "STOP_LOSS_LIMIT"),
            Self::TakeProfitLimit => write!(f, "TAKE_PROFIT_LIMIT"),
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

