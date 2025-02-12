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
pub struct TickerAllUpdate {
    /// Array of detailed market ticker information for all markets.
    #[serde(rename = "tickerAll")]
    pub ticker_all: Vec<models::TickerUpdate>,
}

impl TickerAllUpdate {
    pub fn new(ticker_all: Vec<models::TickerUpdate>) -> TickerAllUpdate {
        TickerAllUpdate {
            ticker_all,
        }
    }
}

