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
pub struct Position {
    /// Market address.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Average entry price determined by a simple average of all entry prices resulting in this position size (e9 format).
    #[serde(rename = "avgEntryPriceE9")]
    pub avg_entry_price_e9: String,
    /// Isolated position leverage (e9 format).
    #[serde(rename = "leverageE9")]
    pub leverage_e9: String,
    /// Liquidation price (e9 format).
    #[serde(rename = "liquidationPriceE9")]
    pub liquidation_price_e9: String,
    /// Mark price (e9 format).
    #[serde(rename = "markPriceE9")]
    pub mark_price_e9: String,
    /// Notional value (e9 format).
    #[serde(rename = "notionalValueE9")]
    pub notional_value_e9: String,
    /// Max notional value at current leverage (e9 format).
    #[serde(rename = "maxNotionalValueE9")]
    pub max_notional_value_e9: String,
    /// Position size (e9 format).
    #[serde(rename = "positionSizeE9")]
    pub position_size_e9: String,
    /// Unrealized profit (e9 format).
    #[serde(rename = "unrealizedPnlE9")]
    pub unrealized_pnl_e9: String,
    #[serde(rename = "positionSide")]
    pub position_side: models::PositionSideEnum,
    /// Initial margin required with current mark price (e9 format).
    #[serde(rename = "initialMarginE9")]
    pub initial_margin_e9: String,
    /// Maintenance margin required with current mark price (e9 format).
    #[serde(rename = "maintMarginE9")]
    pub maint_margin_e9: String,
    /// If the position is isolated.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Margin value present if margin type is isolated (e9 format).
    #[serde(rename = "isolatedMarginE9")]
    pub isolated_margin_e9: String,
    /// Last update time.
    #[serde(rename = "lastUpdatedAtUtcMillis")]
    pub last_updated_at_utc_millis: i64,
}

impl Position {
    pub fn new(symbol: String, avg_entry_price_e9: String, leverage_e9: String, liquidation_price_e9: String, mark_price_e9: String, notional_value_e9: String, max_notional_value_e9: String, position_size_e9: String, unrealized_pnl_e9: String, position_side: models::PositionSideEnum, initial_margin_e9: String, maint_margin_e9: String, is_isolated: bool, isolated_margin_e9: String, last_updated_at_utc_millis: i64) -> Position {
        Position {
            symbol,
            avg_entry_price_e9,
            leverage_e9,
            liquidation_price_e9,
            mark_price_e9,
            notional_value_e9,
            max_notional_value_e9,
            position_size_e9,
            unrealized_pnl_e9,
            position_side,
            initial_margin_e9,
            maint_margin_e9,
            is_isolated,
            isolated_margin_e9,
            last_updated_at_utc_millis,
        }
    }
}

