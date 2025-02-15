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

/// TransactionTypeEnum : Transaction type (what caused the change in the asset balance).
/// Transaction type (what caused the change in the asset balance).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionTypeEnum {
    #[serde(rename = "TRANSFER")]
    Transfer,
    #[serde(rename = "DEPOSIT")]
    Deposit,
    #[serde(rename = "WITHDRAW")]
    Withdraw,
    #[serde(rename = "REALIZED_PNL")]
    RealizedPnl,
    #[serde(rename = "FUNDING_FEE")]
    FundingFee,
    #[serde(rename = "TRADING_FEE")]
    TradingFee,
    #[serde(rename = "TRADING_GAS_FEE")]
    TradingGasFee,
    #[serde(rename = "BONUS")]
    Bonus,

}

impl std::fmt::Display for TransactionTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Transfer => write!(f, "TRANSFER"),
            Self::Deposit => write!(f, "DEPOSIT"),
            Self::Withdraw => write!(f, "WITHDRAW"),
            Self::RealizedPnl => write!(f, "REALIZED_PNL"),
            Self::FundingFee => write!(f, "FUNDING_FEE"),
            Self::TradingFee => write!(f, "TRADING_FEE"),
            Self::TradingGasFee => write!(f, "TRADING_GAS_FEE"),
            Self::Bonus => write!(f, "BONUS"),
        }
    }
}

impl Default for TransactionTypeEnum {
    fn default() -> TransactionTypeEnum {
        Self::Transfer
    }
}

