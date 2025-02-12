use bluefin_api::models::WithdrawRequest;
use sha2::digest::Digest;

use crate::signature::{self, RequestExt, Type};

mod bcs_conversion {
    use crate::signature::parse;
    use crate::{address::Address, signature::Error};
    use bluefin_api::models::WithdrawRequestSignedFields;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct WithdrawRequestSignFieldsBcs {
        pub asset_symbol: String,
        pub account_address: Address,
        pub amount: u64,
        pub salt: u64,
        pub eds_id: Address,
        pub signed_at_utc_millis: u64,
    }

    pub fn convert(
        withdraw_request_sign_fields: &WithdrawRequestSignedFields,
    ) -> Result<WithdrawRequestSignFieldsBcs, Error> {
        Ok(WithdrawRequestSignFieldsBcs {
            asset_symbol: withdraw_request_sign_fields.asset_symbol.clone(),
            account_address: parse(
                &withdraw_request_sign_fields.account_address,
                "account_address",
            )?,
            amount: parse(&withdraw_request_sign_fields.amount_e9, "amount")?,
            salt: parse(&withdraw_request_sign_fields.salt, "salt")?,
            eds_id: parse(&withdraw_request_sign_fields.eds_id, "eds_id")?,
            signed_at_utc_millis: withdraw_request_sign_fields
                .signed_at_utc_millis
                .unsigned_abs(),
        })
    }
}

impl RequestExt for WithdrawRequest {
    fn sign(mut self, private_key: [u8; 32], type_id: Type) -> signature::Result<Self> {
        let bcs_request = bcs::to_bytes(&bcs_conversion::convert(&self.signed_fields)?)
            .map_err(|error| signature::Error::RequestObject(error.to_string()))?;
        let sha_encoded_request = sha2::Sha256::digest(&bcs_request);
        self.request_hash = hex::encode(sha_encoded_request);
        self.signature = signature::sign(private_key, type_id, sha_encoded_request.as_ref())?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signature::testing::verify_signature;
    use bluefin_api::models::{WithdrawRequest, WithdrawRequestSignedFields};
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    #[test]
    fn sign_request_is_successful_ed25519() {
        let request = withdraw_request()
            .sign(SigningKey::generate(&mut OsRng).to_bytes(), Type::Ed25519)
            .unwrap();
        verify_request_signature(request);
    }

    #[test]
    fn sign_request_is_successful_secp256k1() {
        let request = withdraw_request()
            .sign(SigningKey::generate(&mut OsRng).to_bytes(), Type::Secp256k1)
            .unwrap();
        verify_request_signature(request);
    }

    fn verify_request_signature(request: WithdrawRequest) {
        assert!(!request.signature.is_empty());
        assert!(!request.request_hash.is_empty());
        assert!(
            verify_signature(&request.signature.clone(), request, |request| {
                bcs_conversion::convert(&request.signed_fields)
            })
            .is_ok()
        )
    }

    fn withdraw_request() -> WithdrawRequest {
        WithdrawRequest {
            signed_fields: WithdrawRequestSignedFields {
                asset_symbol: "USDC".to_string(),
                account_address:
                    "0x8e78225d72b1d7b1f63e5e9f88f09b12ca66c84e2fc8b91fc10f6a0c51230615".into(),
                amount_e9: "3500000000000".to_string(),
                salt: "1725930601205".to_string(),
                eds_id: "0x8e78225d72b1d7b1f63e5e9f88f09b12ca66c84e2fc8b91fc10f6a0c51230615".into(),
                signed_at_utc_millis: 1725931543867,
            },
            signature: String::new(),
            request_hash: String::new(),
        }
    }
}
