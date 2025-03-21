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
pub struct AccountAuthorizationRequestSignedFields {
    /// The account address of the parent account that is authorizing/deauthorizing this account
    #[serde(rename = "accountAddress")]
    pub account_address: String,
    /// The address of the account that should be authorized/deauthorized
    #[serde(rename = "authorizedAccountAddress")]
    pub authorized_account_address: String,
    /// The random generated salt. Should always be a number
    #[serde(rename = "salt")]
    pub salt: String,
    /// the ID of the internal datastore for the target network
    #[serde(rename = "idsId")]
    pub ids_id: String,
    /// The timestamp when the request was signed
    #[serde(rename = "signedAtMillis")]
    pub signed_at_millis: i64,
}

impl AccountAuthorizationRequestSignedFields {
    pub fn new(account_address: String, authorized_account_address: String, salt: String, ids_id: String, signed_at_millis: i64) -> AccountAuthorizationRequestSignedFields {
        AccountAuthorizationRequestSignedFields {
            account_address,
            authorized_account_address,
            salt,
            ids_id,
            signed_at_millis,
        }
    }
}

