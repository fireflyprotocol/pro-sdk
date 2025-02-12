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

/// AccountStreamMessage : Account stream message for account-related events. The payload depends on the event type. 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum AccountStreamMessage {
    #[serde(rename="AccountOrderUpdate")]
    AccountOrderUpdate {
        #[serde(rename = "reason")]
        reason: models::AccountEventReason,
        #[serde(rename = "payload")]
        payload: models::AccountStreamMessagePayload,
    },
    #[serde(rename="AccountPositionUpdate")]
    AccountPositionUpdate {
        #[serde(rename = "reason")]
        reason: models::AccountEventReason,
        #[serde(rename = "payload")]
        payload: models::AccountStreamMessagePayload,
    },
    #[serde(rename="AccountTradeUpdate")]
    AccountTradeUpdate {
        #[serde(rename = "reason")]
        reason: models::AccountEventReason,
        #[serde(rename = "payload")]
        payload: models::AccountStreamMessagePayload,
    },
    #[serde(rename="AccountTransactionUpdate")]
    AccountTransactionUpdate {
        #[serde(rename = "reason")]
        reason: models::AccountEventReason,
        #[serde(rename = "payload")]
        payload: models::AccountStreamMessagePayload,
    },
    #[serde(rename="AccountUpdate")]
    AccountUpdate {
        #[serde(rename = "reason")]
        reason: models::AccountEventReason,
        #[serde(rename = "payload")]
        payload: models::AccountStreamMessagePayload,
    },
}

impl Default for AccountStreamMessage {
    fn default() -> Self {
        Self::AccountOrderUpdate {
            reason: Default::default(),
            payload: Default::default(),
        }
        
    }
}


