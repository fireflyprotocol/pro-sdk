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

/// Asset2 : Details about an asset in the account.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Asset2 {
    /// The symbol of the asset.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// The quantity of the asset.
    #[serde(rename = "quantityE9")]
    pub quantity_e9: String,
    /// The effective balance of the asset.
    #[serde(rename = "effectiveBalanceE9")]
    pub effective_balance_e9: String,
    /// The maximum quantity that can be withdrawn.
    #[serde(rename = "maxWithdrawQuantityE9")]
    pub max_withdraw_quantity_e9: String,
    /// The timestamp of the last update in milliseconds.
    #[serde(rename = "updatedAtUtcMillis")]
    pub updated_at_utc_millis: i64,
}

impl Asset2 {
    /// Details about an asset in the account.
    pub fn new(symbol: String, quantity_e9: String, effective_balance_e9: String, max_withdraw_quantity_e9: String, updated_at_utc_millis: i64) -> Asset2 {
        Asset2 {
            symbol,
            quantity_e9,
            effective_balance_e9,
            max_withdraw_quantity_e9,
            updated_at_utc_millis,
        }
    }
}

