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
pub struct Trade {
    /// Trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    /// Client order ID.
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// Market address.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Order hash.
    #[serde(rename = "orderHash", skip_serializing_if = "Option::is_none")]
    pub order_hash: Option<String>,
    #[serde(rename = "tradeType")]
    pub trade_type: models::TradeTypeEnum,
    #[serde(rename = "side")]
    pub side: models::TradeSideEnum,
    /// Indicates if the user was a maker to the trade.
    #[serde(rename = "isMaker")]
    pub is_maker: bool,
    /// Trade price (e9 format).
    #[serde(rename = "priceE9")]
    pub price_e9: String,
    /// Trade quantity (e9 format).
    #[serde(rename = "quantityE9")]
    pub quantity_e9: String,
    /// Quote quantity (e9 format).
    #[serde(rename = "quoteQuantityE9")]
    pub quote_quantity_e9: String,
    /// Realized profit and loss (e9 format).
    #[serde(rename = "realizedPnlE9", skip_serializing_if = "Option::is_none")]
    pub realized_pnl_e9: Option<String>,
    #[serde(rename = "positionSide")]
    pub position_side: models::PositionSideEnum,
    /// Trading fee (e9 format).
    #[serde(rename = "tradingFeeE9")]
    pub trading_fee_e9: String,
    /// Asset used for trading fee.
    #[serde(rename = "tradingFeeAsset")]
    pub trading_fee_asset: TradingFeeAsset,
    /// Gas fee.
    #[serde(rename = "gasFeeE9", skip_serializing_if = "Option::is_none")]
    pub gas_fee_e9: Option<f32>,
    /// Asset used for gas fee.
    #[serde(rename = "gasFeeAsset", skip_serializing_if = "Option::is_none")]
    pub gas_fee_asset: Option<String>,
    /// Trade timestamp in milliseconds since Unix epoch.
    #[serde(rename = "executedAtMillis")]
    pub executed_at_millis: i64,
}

impl Trade {
    pub fn new(trade_id: String, trade_type: models::TradeTypeEnum, side: models::TradeSideEnum, is_maker: bool, price_e9: String, quantity_e9: String, quote_quantity_e9: String, position_side: models::PositionSideEnum, trading_fee_e9: String, trading_fee_asset: TradingFeeAsset, executed_at_millis: i64) -> Trade {
        Trade {
            trade_id,
            client_order_id: None,
            symbol: None,
            order_hash: None,
            trade_type,
            side,
            is_maker,
            price_e9,
            quantity_e9,
            quote_quantity_e9,
            realized_pnl_e9: None,
            position_side,
            trading_fee_e9,
            trading_fee_asset,
            gas_fee_e9: None,
            gas_fee_asset: None,
            executed_at_millis,
        }
    }
}
/// Asset used for trading fee.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TradingFeeAsset {
    #[serde(rename = "USDC")]
    Usdc,
    #[serde(rename = "BLUE")]
    Blue,
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,
}

impl Default for TradingFeeAsset {
    fn default() -> TradingFeeAsset {
        Self::Usdc
    }
}

