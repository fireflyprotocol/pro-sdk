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
pub struct MarkPriceUpdate {
    /// The symbol of the market.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// The price in scientific notation with 9 decimal places of precision.
    #[serde(rename = "priceE9")]
    pub price_e9: String,
    #[serde(rename = "source")]
    pub source: Source,
    /// The timestamp of the price update.
    #[serde(rename = "updatedAtUtcMillis")]
    pub updated_at_utc_millis: i64,
}

impl MarkPriceUpdate {
    pub fn new(symbol: String, price_e9: String, source: Source, updated_at_utc_millis: i64) -> MarkPriceUpdate {
        MarkPriceUpdate {
            symbol,
            price_e9,
            source,
            updated_at_utc_millis,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "Mark")]
    Mark,
}

impl Default for Source {
    fn default() -> Source {
        Self::Mark
    }
}

