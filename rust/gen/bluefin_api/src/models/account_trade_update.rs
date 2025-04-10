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

/// AccountTradeUpdate : Details about a trade in the account.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountTradeUpdate {
    #[serde(rename = "trade")]
    pub trade: models::Trade,
}

impl AccountTradeUpdate {
    /// Details about a trade in the account.
    pub fn new(trade: models::Trade) -> AccountTradeUpdate {
        AccountTradeUpdate {
            trade,
        }
    }
}

