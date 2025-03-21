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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum KlineInterval {
    #[serde(rename = "1m")]
    Variant1m,
    #[serde(rename = "3m")]
    Variant3m,
    #[serde(rename = "5m")]
    Variant5m,
    #[serde(rename = "15m")]
    Variant15m,
    #[serde(rename = "30m")]
    Variant30m,
    #[serde(rename = "1h")]
    Variant1h,
    #[serde(rename = "2h")]
    Variant2h,
    #[serde(rename = "4h")]
    Variant4h,
    #[serde(rename = "6h")]
    Variant6h,
    #[serde(rename = "8h")]
    Variant8h,
    #[serde(rename = "12h")]
    Variant12h,
    #[serde(rename = "1d")]
    Variant1d,
    #[serde(rename = "1w")]
    Variant1w,
    #[serde(rename = "1Mo")]
    Variant1Mo,
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,

}

impl std::fmt::Display for KlineInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant1m => write!(f, "1m"),
            Self::Variant3m => write!(f, "3m"),
            Self::Variant5m => write!(f, "5m"),
            Self::Variant15m => write!(f, "15m"),
            Self::Variant30m => write!(f, "30m"),
            Self::Variant1h => write!(f, "1h"),
            Self::Variant2h => write!(f, "2h"),
            Self::Variant4h => write!(f, "4h"),
            Self::Variant6h => write!(f, "6h"),
            Self::Variant8h => write!(f, "8h"),
            Self::Variant12h => write!(f, "12h"),
            Self::Variant1d => write!(f, "1d"),
            Self::Variant1w => write!(f, "1w"),
            Self::Variant1Mo => write!(f, "1Mo"),
            Self::Unspecified => write!(f, "UNSPECIFIED"),
        }
    }
}

impl Default for KlineInterval {
    fn default() -> KlineInterval {
        Self::Variant1m
    }
}

