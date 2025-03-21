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
pub struct OrderbookDepthResponse {
    /// Market symbol.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Count indicating the number of changes in orderbook state.
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: i64,
    /// Timestamp at which the last change in orderbook state took place, in milliseconds.
    #[serde(rename = "updatedAtMillis")]
    pub updated_at_millis: i64,
    /// The time at which the orderbook server sent the response, in milliseconds.
    #[serde(rename = "responseSentAtMillis")]
    pub response_sent_at_millis: i64,
    /// The best bid price on orderbook at the moment (e9 format).
    #[serde(rename = "bestBidPriceE9")]
    pub best_bid_price_e9: String,
    /// The best bid quantity on orderbook at the moment (e9 format).
    #[serde(rename = "bestBidQuantityE9")]
    pub best_bid_quantity_e9: String,
    /// The best ask price on orderbook at the moment (e9 format).
    #[serde(rename = "bestAskPriceE9")]
    pub best_ask_price_e9: String,
    /// The best ask quantity on orderbook at the moment (e9 format).
    #[serde(rename = "bestAskQuantityE9")]
    pub best_ask_quantity_e9: String,
    /// Bids to be filled. Index 0 is price, index 1 is quantity at price bin. Prices are in e9 format.
    #[serde(rename = "bidsE9")]
    pub bids_e9: Vec<Vec<String>>,
    /// Asks to be filled. Index 0 is price, index 1 is quantity at price bin. Prices are in e9 format.
    #[serde(rename = "asksE9")]
    pub asks_e9: Vec<Vec<String>>,
}

impl OrderbookDepthResponse {
    pub fn new(symbol: String, last_update_id: i64, updated_at_millis: i64, response_sent_at_millis: i64, best_bid_price_e9: String, best_bid_quantity_e9: String, best_ask_price_e9: String, best_ask_quantity_e9: String, bids_e9: Vec<Vec<String>>, asks_e9: Vec<Vec<String>>) -> OrderbookDepthResponse {
        OrderbookDepthResponse {
            symbol,
            last_update_id,
            updated_at_millis,
            response_sent_at_millis,
            best_bid_price_e9,
            best_bid_quantity_e9,
            best_ask_price_e9,
            best_ask_quantity_e9,
            bids_e9,
            asks_e9,
        }
    }
}

