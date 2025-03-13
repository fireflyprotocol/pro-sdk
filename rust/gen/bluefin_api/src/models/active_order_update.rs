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

/// ActiveOrderUpdate : Information about an order update.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveOrderUpdate {
    /// The unique hash of the order.
    #[serde(rename = "orderHash")]
    pub order_hash: String,
    /// The client-provided order ID.
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// The symbol of the market.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// The address of the account.
    #[serde(rename = "accountAddress")]
    pub account_address: String,
    /// The price of the order in scientific notation with 9 decimal places.
    #[serde(rename = "priceE9")]
    pub price_e9: String,
    /// The quantity of the order in scientific notation with 9 decimal places.
    #[serde(rename = "quantityE9")]
    pub quantity_e9: String,
    /// The filled quantity of the order in scientific notation with 9 decimal places.
    #[serde(rename = "filledQuantityE9")]
    pub filled_quantity_e9: String,
    #[serde(rename = "side")]
    pub side: models::Side,
    /// The leverage of the order in scientific notation with 9 decimal places.
    #[serde(rename = "leverageE9")]
    pub leverage_e9: String,
    /// Indicates if the order is isolated.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// A unique salt for the order.
    #[serde(rename = "salt")]
    pub salt: String,
    /// The expiration timestamp of the order in milliseconds.
    #[serde(rename = "expiresAtMillis")]
    pub expires_at_millis: i64,
    /// The signing timestamp of the order in milliseconds.
    #[serde(rename = "signedAtMillis")]
    pub signed_at_millis: i64,
    #[serde(rename = "type")]
    pub r#type: models::OrderType1,
    /// Indicates if the order is reduce-only.
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    /// Indicates if the order is post-only.
    #[serde(rename = "postOnly")]
    pub post_only: bool,
    #[serde(rename = "timeInForce")]
    pub time_in_force: models::OrderTimeInForce1,
    /// The trigger price for stop-limit or stop-market orders.
    #[serde(rename = "triggerPriceE9", skip_serializing_if = "Option::is_none")]
    pub trigger_price_e9: Option<String>,
    #[serde(rename = "status")]
    pub status: models::OrderStatus1,
    #[serde(rename = "selfTradePreventionType")]
    pub self_trade_prevention_type: models::SelfTradePreventionType1,
    /// The timestamp when the order was placed, in milliseconds.
    #[serde(rename = "createdAtMillis")]
    pub created_at_millis: i64,
    /// The timestamp of the last update of the order in milliseconds.
    #[serde(rename = "updatedAtMillis")]
    pub updated_at_millis: i64,
}

impl ActiveOrderUpdate {
    /// Information about an order update.
    pub fn new(order_hash: String, symbol: String, account_address: String, price_e9: String, quantity_e9: String, filled_quantity_e9: String, side: models::Side, leverage_e9: String, is_isolated: bool, salt: String, expires_at_millis: i64, signed_at_millis: i64, r#type: models::OrderType1, reduce_only: bool, post_only: bool, time_in_force: models::OrderTimeInForce1, status: models::OrderStatus1, self_trade_prevention_type: models::SelfTradePreventionType1, created_at_millis: i64, updated_at_millis: i64) -> ActiveOrderUpdate {
        ActiveOrderUpdate {
            order_hash,
            client_order_id: None,
            symbol,
            account_address,
            price_e9,
            quantity_e9,
            filled_quantity_e9,
            side,
            leverage_e9,
            is_isolated,
            salt,
            expires_at_millis,
            signed_at_millis,
            r#type,
            reduce_only,
            post_only,
            time_in_force,
            trigger_price_e9: None,
            status,
            self_trade_prevention_type,
            created_at_millis,
            updated_at_millis,
        }
    }
}

