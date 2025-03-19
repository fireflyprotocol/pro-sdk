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
pub struct AssetConfig {
    /// The bank address of the asset.
    #[serde(rename = "assetType")]
    pub asset_type: String,
    /// Asset symbol.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Default precision for rendering this asset.
    #[serde(rename = "decimals")]
    pub decimals: i64,
    /// Weight applied to asset to use as margin in Multi-Assets mode.
    #[serde(rename = "weight")]
    pub weight: String,
    /// Indicates if the asset can be used as margin in Multi-Assets mode.
    #[serde(rename = "marginAvailable")]
    pub margin_available: bool,
}

impl AssetConfig {
    pub fn new(asset_type: String, symbol: String, decimals: i64, weight: String, margin_available: bool) -> AssetConfig {
        AssetConfig {
            asset_type,
            symbol,
            decimals,
            weight,
            margin_available,
        }
    }
}

