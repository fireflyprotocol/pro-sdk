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
pub struct OrderbookDiffDepthUpdate {
    /// The timestamp of the orderbook update.
    #[serde(rename = "updatedAtMillis")]
    pub updated_at_millis: i64,
    /// The symbol of the market for the orderbook update.
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "bidsE9")]
    pub bids_e9: Vec<Vec<String>>,
    #[serde(rename = "asksE9")]
    pub asks_e9: Vec<Vec<String>>,
    /// The ID of the first update in this batch.
    #[serde(rename = "firstUpdateId")]
    pub first_update_id: i64,
    /// The ID of the last update in this batch.
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: i64,
}

impl OrderbookDiffDepthUpdate {
    pub fn new(updated_at_millis: i64, symbol: String, bids_e9: Vec<Vec<String>>, asks_e9: Vec<Vec<String>>, first_update_id: i64, last_update_id: i64) -> OrderbookDiffDepthUpdate {
        OrderbookDiffDepthUpdate {
            updated_at_millis,
            symbol,
            bids_e9,
            asks_e9,
            first_update_id,
            last_update_id,
        }
    }
}

