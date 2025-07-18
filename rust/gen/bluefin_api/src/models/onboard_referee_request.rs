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
pub struct OnboardRefereeRequest {
    /// Referral code of the parent affiliate
    #[serde(rename = "code")]
    pub code: String,
}

impl OnboardRefereeRequest {
    pub fn new(code: String) -> OnboardRefereeRequest {
        OnboardRefereeRequest {
            code,
        }
    }
}

