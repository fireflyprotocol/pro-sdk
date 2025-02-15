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
pub struct Account {
    #[serde(rename = "tradingFees")]
    pub trading_fees: models::TradingFees,
    /// If the user can trade.
    #[serde(rename = "canTrade")]
    pub can_trade: bool,
    /// If the current user can deposit to the account.
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,
    /// If the current user can withdraw from the account.
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,
    /// Total effective balance in USD (e9 format).
    #[serde(rename = "totalEffectiveBalanceE9")]
    pub total_effective_balance_e9: String,
    /// The sum of initial margin required across all cross positions (e9 format).
    #[serde(rename = "totalInitialMarginRequiredE9")]
    pub total_initial_margin_required_e9: String,
    /// The sum of initial margin required across all open orders (e9 format).
    #[serde(rename = "totalOpenOrderInitialMarginRequiredE9")]
    pub total_open_order_initial_margin_required_e9: String,
    /// The amount of margin available to open new positions and orders (e9 format).
    #[serde(rename = "initialMarginAvailableE9")]
    pub initial_margin_available_e9: String,
    /// The sum of maintenance margin required across all cross positions (e9 format).
    #[serde(rename = "totalMaintMarginRequiredE9")]
    pub total_maint_margin_required_e9: String,
    /// The amount of margin available before liquidation (e9 format).
    #[serde(rename = "maintMarginAvailableE9")]
    pub maint_margin_available_e9: String,
    /// The ratio of the maintenance margin required to the account value (e9 format).
    #[serde(rename = "accountMaintMarginRatioE9")]
    pub account_maint_margin_ratio_e9: String,
    /// The leverage of the account (e9 format).
    #[serde(rename = "accountLeverageE9")]
    pub account_leverage_e9: String,
    /// Total unrealized profit (e9 format).
    #[serde(rename = "totalUnrealizedPnlE9")]
    pub total_unrealized_pnl_e9: String,
    /// Unrealized profit of crossed positions (e9 format).
    #[serde(rename = "totalCrossUnrealizedPnlE9")]
    pub total_cross_unrealized_pnl_e9: String,
    /// Last update time in milliseconds since Unix epoch.
    #[serde(rename = "lastUpdatedAtUtcMillis")]
    pub last_updated_at_utc_millis: i64,
    #[serde(rename = "assets")]
    pub assets: Vec<models::Asset>,
    #[serde(rename = "positions")]
    pub positions: Vec<models::Position>,
}

impl Account {
    pub fn new(trading_fees: models::TradingFees, can_trade: bool, can_deposit: bool, can_withdraw: bool, total_effective_balance_e9: String, total_initial_margin_required_e9: String, total_open_order_initial_margin_required_e9: String, initial_margin_available_e9: String, total_maint_margin_required_e9: String, maint_margin_available_e9: String, account_maint_margin_ratio_e9: String, account_leverage_e9: String, total_unrealized_pnl_e9: String, total_cross_unrealized_pnl_e9: String, last_updated_at_utc_millis: i64, assets: Vec<models::Asset>, positions: Vec<models::Position>) -> Account {
        Account {
            trading_fees,
            can_trade,
            can_deposit,
            can_withdraw,
            total_effective_balance_e9,
            total_initial_margin_required_e9,
            total_open_order_initial_margin_required_e9,
            initial_margin_available_e9,
            total_maint_margin_required_e9,
            maint_margin_available_e9,
            account_maint_margin_ratio_e9,
            account_leverage_e9,
            total_unrealized_pnl_e9,
            total_cross_unrealized_pnl_e9,
            last_updated_at_utc_millis,
            assets,
            positions,
        }
    }
}

