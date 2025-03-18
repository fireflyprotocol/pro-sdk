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
pub struct MarketPriceUpdate {
    /// The symbol of the market.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// The price in scientific notation with 9 decimal places of precision.
    #[serde(rename = "priceE9")]
    pub price_e9: String,
    #[serde(rename = "source")]
    pub source: Source,
    /// The timestamp of the price update.
    #[serde(rename = "updatedAtMillis")]
    pub updated_at_millis: i64,
}

impl MarketPriceUpdate {
    pub fn new(symbol: String, price_e9: String, source: Source, updated_at_millis: i64) -> MarketPriceUpdate {
        MarketPriceUpdate {
            symbol,
            price_e9,
            source,
            updated_at_millis,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "Market")]
    Market,
}

impl Default for Source {
    fn default() -> Source {
        Self::Market
    }
}

