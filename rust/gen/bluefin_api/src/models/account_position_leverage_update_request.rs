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
pub struct AccountPositionLeverageUpdateRequest {
    #[serde(rename = "signedFields")]
    pub signed_fields: models::AccountPositionLeverageUpdateRequestSignedFields,
    /// The signature of the request, encoded from the signedFields
    #[serde(rename = "signature")]
    pub signature: String,
}

impl AccountPositionLeverageUpdateRequest {
    pub fn new(signed_fields: models::AccountPositionLeverageUpdateRequestSignedFields, signature: String) -> AccountPositionLeverageUpdateRequest {
        AccountPositionLeverageUpdateRequest {
            signed_fields,
            signature,
        }
    }
}

