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

/// AccountEventType : The type of account-related event.
/// The type of account-related event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountEventType {
    #[serde(rename = "AccountUpdate")]
    AccountUpdate,
    #[serde(rename = "AccountTradeUpdate")]
    AccountTradeUpdate,
    #[serde(rename = "AccountAggregatedTradeUpdate")]
    AccountAggregatedTradeUpdate,
    #[serde(rename = "AccountOrderUpdate")]
    AccountOrderUpdate,
    #[serde(rename = "AccountPositionUpdate")]
    AccountPositionUpdate,
    #[serde(rename = "AccountTransactionUpdate")]
    AccountTransactionUpdate,
    #[serde(rename = "AccountCommandFailureUpdate")]
    AccountCommandFailureUpdate,

}

impl std::fmt::Display for AccountEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AccountUpdate => write!(f, "AccountUpdate"),
            Self::AccountTradeUpdate => write!(f, "AccountTradeUpdate"),
            Self::AccountAggregatedTradeUpdate => write!(f, "AccountAggregatedTradeUpdate"),
            Self::AccountOrderUpdate => write!(f, "AccountOrderUpdate"),
            Self::AccountPositionUpdate => write!(f, "AccountPositionUpdate"),
            Self::AccountTransactionUpdate => write!(f, "AccountTransactionUpdate"),
            Self::AccountCommandFailureUpdate => write!(f, "AccountCommandFailureUpdate"),
        }
    }
}

impl Default for AccountEventType {
    fn default() -> AccountEventType {
        Self::AccountUpdate
    }
}

