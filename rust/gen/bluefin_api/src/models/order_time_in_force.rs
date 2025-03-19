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

/// OrderTimeInForce : The time-in-force policy for the order. By default, all orders are GTT. UNSPECIFIED is set to default.  GTT: Good Til Time  IOC: Immediate Or Cancel  FOK: Fill Or Kill     
/// The time-in-force policy for the order. By default, all orders are GTT. UNSPECIFIED is set to default.  GTT: Good Til Time  IOC: Immediate Or Cancel  FOK: Fill Or Kill     
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderTimeInForce {
    #[serde(rename = "GTT")]
    Gtt,
    #[serde(rename = "IOC")]
    Ioc,
    #[serde(rename = "FOK")]
    Fok,
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,

}

impl std::fmt::Display for OrderTimeInForce {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Gtt => write!(f, "GTT"),
            Self::Ioc => write!(f, "IOC"),
            Self::Fok => write!(f, "FOK"),
            Self::Unspecified => write!(f, "UNSPECIFIED"),
        }
    }
}

impl Default for OrderTimeInForce {
    fn default() -> OrderTimeInForce {
        Self::Gtt
    }
}

