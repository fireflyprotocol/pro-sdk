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
pub struct WithdrawRequest {
    #[serde(rename = "signedFields")]
    pub signed_fields: models::WithdrawRequestSignedFields,
    /// The signature of the request, encoded from the signedFields
    #[serde(rename = "signature")]
    pub signature: String,
}

impl WithdrawRequest {
    pub fn new(signed_fields: models::WithdrawRequestSignedFields, signature: String) -> WithdrawRequest {
        WithdrawRequest {
            signed_fields,
            signature,
        }
    }
}

