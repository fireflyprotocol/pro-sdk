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
pub struct ExchangeInfoResponse {
    /// List of assets available on the exchange.
    #[serde(rename = "assets")]
    pub assets: Vec<models::Asset1>,
    #[serde(rename = "contractsConfig", skip_serializing_if = "Option::is_none")]
    pub contracts_config: Option<models::ContractsConfig>,
    /// List of markets available on the exchange.
    #[serde(rename = "markets")]
    pub markets: Vec<models::Market>,
    /// Current gas fee set for subsidized trades (e9 format)
    #[serde(rename = "tradingGasFeeE9")]
    pub trading_gas_fee_e9: String,
    /// Server time in milliseconds.
    #[serde(rename = "serverTimeAtMillis")]
    pub server_time_at_millis: i64,
    /// Timezone of the exchange.
    #[serde(rename = "timezone")]
    pub timezone: String,
}

impl ExchangeInfoResponse {
    pub fn new(assets: Vec<models::Asset1>, markets: Vec<models::Market>, trading_gas_fee_e9: String, server_time_at_millis: i64, timezone: String) -> ExchangeInfoResponse {
        ExchangeInfoResponse {
            assets,
            contracts_config: None,
            markets,
            trading_gas_fee_e9,
            server_time_at_millis,
            timezone,
        }
    }
}

