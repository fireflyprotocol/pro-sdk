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

/// ContractsConfig : Contract configuration for the exchange.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractsConfig {
    /// External Data Store Address
    #[serde(rename = "edsId")]
    pub eds_id: String,
    /// External Data Store Address
    #[serde(rename = "idsId")]
    pub ids_id: String,
    /// Network environment
    #[serde(rename = "network")]
    pub network: Network,
    /// Base contract address
    #[serde(rename = "baseContractAddress")]
    pub base_contract_address: String,
    /// Current contract address
    #[serde(rename = "currentContractAddress")]
    pub current_contract_address: String,
    #[serde(rename = "operators")]
    pub operators: models::Operators,
}

impl ContractsConfig {
    /// Contract configuration for the exchange.
    pub fn new(eds_id: String, ids_id: String, network: Network, base_contract_address: String, current_contract_address: String, operators: models::Operators) -> ContractsConfig {
        ContractsConfig {
            eds_id,
            ids_id,
            network,
            base_contract_address,
            current_contract_address,
            operators,
        }
    }
}
/// Network environment
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Network {
    #[serde(rename = "mainnet")]
    Mainnet,
    #[serde(rename = "testnet")]
    Testnet,
    #[serde(rename = "devnet")]
    Devnet,
}

impl Default for Network {
    fn default() -> Network {
        Self::Mainnet
    }
}

