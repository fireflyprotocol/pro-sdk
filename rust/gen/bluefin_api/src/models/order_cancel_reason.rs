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

/// OrderCancelReason : The reason for an order cancellation.
/// The reason for an order cancellation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderCancelReason {
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "INSUFFICIENT_MARGIN")]
    InsufficientMargin,
    #[serde(rename = "DUPLICATE_ORDER")]
    DuplicateOrder,
    #[serde(rename = "POST_ONLY_WOULD_TRADE")]
    PostOnlyWouldTrade,
    #[serde(rename = "INVALID_SYMBOL")]
    InvalidSymbol,
    #[serde(rename = "SIGNED_AT_TOO_OLD")]
    SignedAtTooOld,
    #[serde(rename = "ORDER_EXPIRED")]
    OrderExpired,
    #[serde(rename = "INVALID_LEVERAGE")]
    InvalidLeverage,
    #[serde(rename = "INVALID_INPUT")]
    InvalidInput,
    #[serde(rename = "PRICE_OUT_OF_BOUND")]
    PriceOutOfBound,
    #[serde(rename = "QUANTITY_OUT_OF_BOUND")]
    QuantityOutOfBound,
    #[serde(rename = "PRICE_OUT_OF_TICK_SIZE")]
    PriceOutOfTickSize,
    #[serde(rename = "QUANTITY_OUT_OF_STEP_SIZE")]
    QuantityOutOfStepSize,
    #[serde(rename = "REDUCE_ONLY_WOULD_OPEN")]
    ReduceOnlyWouldOpen,
    #[serde(rename = "TOO_MANY_OPEN_ORDERS_ON_MARKET")]
    TooManyOpenOrdersOnMarket,
    #[serde(rename = "USER_CANCELLED")]
    UserCancelled,
    #[serde(rename = "SELF_TRADE_PREVENTION")]
    SelfTradePrevention,
    #[serde(rename = "LEVERAGE_UPDATE")]
    LeverageUpdate,

}

impl std::fmt::Display for OrderCancelReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "UNSPECIFIED"),
            Self::InsufficientMargin => write!(f, "INSUFFICIENT_MARGIN"),
            Self::DuplicateOrder => write!(f, "DUPLICATE_ORDER"),
            Self::PostOnlyWouldTrade => write!(f, "POST_ONLY_WOULD_TRADE"),
            Self::InvalidSymbol => write!(f, "INVALID_SYMBOL"),
            Self::SignedAtTooOld => write!(f, "SIGNED_AT_TOO_OLD"),
            Self::OrderExpired => write!(f, "ORDER_EXPIRED"),
            Self::InvalidLeverage => write!(f, "INVALID_LEVERAGE"),
            Self::InvalidInput => write!(f, "INVALID_INPUT"),
            Self::PriceOutOfBound => write!(f, "PRICE_OUT_OF_BOUND"),
            Self::QuantityOutOfBound => write!(f, "QUANTITY_OUT_OF_BOUND"),
            Self::PriceOutOfTickSize => write!(f, "PRICE_OUT_OF_TICK_SIZE"),
            Self::QuantityOutOfStepSize => write!(f, "QUANTITY_OUT_OF_STEP_SIZE"),
            Self::ReduceOnlyWouldOpen => write!(f, "REDUCE_ONLY_WOULD_OPEN"),
            Self::TooManyOpenOrdersOnMarket => write!(f, "TOO_MANY_OPEN_ORDERS_ON_MARKET"),
            Self::UserCancelled => write!(f, "USER_CANCELLED"),
            Self::SelfTradePrevention => write!(f, "SELF_TRADE_PREVENTION"),
            Self::LeverageUpdate => write!(f, "LEVERAGE_UPDATE"),
        }
    }
}

impl Default for OrderCancelReason {
    fn default() -> OrderCancelReason {
        Self::Unspecified
    }
}

