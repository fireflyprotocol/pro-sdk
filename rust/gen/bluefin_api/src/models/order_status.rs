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

/// OrderStatus : The statuses of the Orders  PENDING: The Order has been acknowledged and is currently pending  OPEN: The Order has passed all checks and is open on the order book.  PARTIALLY_FILLED: The Order has been matched for less than its full quantity. Part of the Order got filled and the rest is still OPEN  FILLED: The Order has been fully matched and filled and is no longer on the order book and no longer cancellable.  CANCELLED: The Order has been cancelled and is no longer able to be filled. It has been taken off the Order Book  REJECTED: The Order has been rejected and has not been put on the order book. 
/// The statuses of the Orders  PENDING: The Order has been acknowledged and is currently pending  OPEN: The Order has passed all checks and is open on the order book.  PARTIALLY_FILLED: The Order has been matched for less than its full quantity. Part of the Order got filled and the rest is still OPEN  FILLED: The Order has been fully matched and filled and is no longer on the order book and no longer cancellable.  CANCELLED: The Order has been cancelled and is no longer able to be filled. It has been taken off the Order Book  REJECTED: The Order has been rejected and has not been put on the order book. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "FILLED")]
    Filled,
    #[serde(rename = "PARTIALLY_FILLED")]
    PartiallyFilled,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "REJECTED")]
    Rejected,

}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "OPEN"),
            Self::Pending => write!(f, "PENDING"),
            Self::Filled => write!(f, "FILLED"),
            Self::PartiallyFilled => write!(f, "PARTIALLY_FILLED"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::Rejected => write!(f, "REJECTED"),
        }
    }
}

impl Default for OrderStatus {
    fn default() -> OrderStatus {
        Self::Open
    }
}

