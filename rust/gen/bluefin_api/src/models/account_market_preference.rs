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
pub struct AccountMarketPreference {
    #[serde(rename = "marginType", skip_serializing_if = "Option::is_none")]
    pub margin_type: Option<models::MarginType>,
    /// User set leverage (e.g., 10x).
    #[serde(rename = "setLeverage", skip_serializing_if = "Option::is_none")]
    pub set_leverage: Option<i32>,
}

impl AccountMarketPreference {
    pub fn new() -> AccountMarketPreference {
        AccountMarketPreference {
            margin_type: None,
            set_leverage: None,
        }
    }
}

