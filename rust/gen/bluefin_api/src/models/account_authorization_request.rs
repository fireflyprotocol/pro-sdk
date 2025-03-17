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
pub struct AccountAuthorizationRequest {
    #[serde(rename = "signedFields")]
    pub signed_fields: models::AccountAuthorizationRequestSignedFields,
    /// The signature of the request, encoded from the signedFields
    #[serde(rename = "signature")]
    pub signature: String,
}

impl AccountAuthorizationRequest {
    pub fn new(signed_fields: models::AccountAuthorizationRequestSignedFields, signature: String) -> AccountAuthorizationRequest {
        AccountAuthorizationRequest {
            signed_fields,
            signature,
        }
    }
}

