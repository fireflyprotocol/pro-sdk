use crate::signature::{self, PrivateKey, RequestExt, Type};
use bluefin_api::models::AccountPositionLeverageUpdateRequest;
use sha2::{Digest, Sha256};

pub mod bcs_conversion {
    use crate::address::Address;
    use crate::signature;
    use crate::signature::parse;
    use bluefin_api::models::AccountPositionLeverageUpdateRequestSignedFields;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct UpdateAccountPositionLeverageRequestBcs {
        pub account_address: Address,
        pub symbol: String,
        pub leverage_e9: u64,
        pub salt: u64,
        pub ids_id: Address,
        pub signed_at_utc_millis: u64,
    }

    pub fn convert(
        signed_fields: &AccountPositionLeverageUpdateRequestSignedFields,
    ) -> signature::Result<UpdateAccountPositionLeverageRequestBcs> {
        Ok(UpdateAccountPositionLeverageRequestBcs {
            account_address: parse(&signed_fields.account_address, "account_address")?,
            symbol: parse(&signed_fields.symbol, "symbol")?,
            leverage_e9: parse(&signed_fields.leverage_e9, "leverage_e9")?,
            salt: parse(&signed_fields.salt, "salt")?,
            ids_id: parse(&signed_fields.ids_id, "ids_id")?,
            signed_at_utc_millis: signed_fields.signed_at_utc_millis.unsigned_abs(),
        })
    }
}

impl RequestExt for AccountPositionLeverageUpdateRequest {
    fn sign(mut self, private_key: PrivateKey, type_id: Type) -> signature::Result<Self> {
        let bcs_request = bcs::to_bytes(&bcs_conversion::convert(&self.signed_fields)?)
            .map_err(|error| signature::Error::RequestObject(error.to_string()))?;
        let sha_encoded_request = Sha256::digest(&bcs_request);
        self.request_hash = hex::encode(sha_encoded_request);
        self.signature = signature::sign(private_key, type_id, sha_encoded_request.as_ref())?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use bluefin_api::models::AccountPositionLeverageUpdateRequestSignedFields;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    use super::*;
    use crate::signature::testing::verify_signature;

    fn verify_request_signature(request: AccountPositionLeverageUpdateRequest) {
        assert!(!request.signature.is_empty());
        assert!(!request.request_hash.is_empty());
        assert!(
            verify_signature(&request.signature.clone(), request, |request| {
                bcs_conversion::convert(&request.signed_fields)
            })
            .is_ok()
        )
    }

    #[test]
    fn sign_request_is_successful_ed25519() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let request = update_leverage_request()
            .sign(signing_key.to_bytes(), Type::Ed25519)
            .unwrap();
        verify_request_signature(request);
    }

    #[test]
    fn sign_request_is_successful_secp256k1() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let request = update_leverage_request()
            .sign(signing_key.to_bytes(), Type::Secp256k1)
            .unwrap();
        verify_request_signature(request);
    }

    fn update_leverage_request() -> AccountPositionLeverageUpdateRequest {
        AccountPositionLeverageUpdateRequest {
            signed_fields: AccountPositionLeverageUpdateRequestSignedFields {
                symbol: "ETH-PERP".into(),
                account_address:
                    "0x8e78225d72b1d7b1f63e5e9f88f09b12ca66c84e2fc8b91fc10f6a0c51230615".into(),
                leverage_e9: "0".into(),
                salt: "1725930601205".into(),
                ids_id: "0x8e78225d72b1d7b1f63e5e9f88f09b12ca66c84e2fc8b91fc10f6a0c51230615".into(),
                signed_at_utc_millis: 1725931543867,
            },
            signature: String::new(),
            request_hash: String::new(),
        }
    }
}
