use crate::signature::{self, PrivateKey, RequestExt};
use bluefin_api::models::CreateOrderRequest;

use crate::signature::Type;
use sha2::{Digest, Sha256};

pub mod bcs_conversion {
    use bluefin_api::models::CreateOrderRequestSignedFields;
    use serde::Serialize;

    use crate::address::Address;
    use crate::signature;
    use crate::signature::parse;

    // On-Chain compatible fields for BCS encoding purposes
    #[derive(Serialize)]
    pub struct CreateOrderRequestSignFieldsBcs {
        pub symbol: String,
        pub account_address: Address,
        pub price: u64,
        pub quantity: u64,
        pub leverage: u64,
        pub side: String,
        pub is_isolated: bool,
        pub expires_at_utc_millis: u64,
        pub salt: u64,
        pub ids_id: Address,
        pub signed_at_utc_millis: u64,
    }

    pub fn convert(
        signed_fields: &CreateOrderRequestSignedFields,
    ) -> signature::Result<CreateOrderRequestSignFieldsBcs> {
        Ok(CreateOrderRequestSignFieldsBcs {
            symbol: parse(&signed_fields.symbol, "symbol")?,
            account_address: parse(&signed_fields.account_address, "account_address")?,
            price: parse(&signed_fields.price_e9, "price")?,
            quantity: parse(&signed_fields.quantity_e9, "quantity")?,
            leverage: parse(&signed_fields.leverage_e9, "leverage")?,
            side: signed_fields.side.to_string(),
            is_isolated: signed_fields.is_isolated,
            expires_at_utc_millis: signed_fields.expires_at_utc_millis.unsigned_abs(),
            salt: parse(&signed_fields.salt, "salt")?,
            ids_id: parse(&signed_fields.ids_id, "ids_id")?,
            signed_at_utc_millis: signed_fields.signed_at_utc_millis.unsigned_abs(),
        })
    }
}

impl RequestExt for CreateOrderRequest {
    fn sign(mut self, private_key: PrivateKey, type_id: Type) -> signature::Result<Self> {
        let bcs_request = bcs::to_bytes(&bcs_conversion::convert(&self.signed_fields)?)
            .map_err(|error| signature::Error::RequestObject(error.to_string()))?;
        let sha_encoded_request = Sha256::digest(&bcs_request);
        self.order_hash = hex::encode(sha_encoded_request);
        self.signature = signature::sign(private_key, type_id, sha_encoded_request.as_ref())?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use bluefin_api::models::{CreateOrderRequest, CreateOrderRequestSignedFields, OrderSide};
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    use super::*;
    use crate::signature::testing::verify_signature;

    fn verify_request_signature(request: CreateOrderRequest) {
        assert!(!request.signature.is_empty());
        assert!(!request.order_hash.is_empty());
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
        let request = create_order_request()
            .sign(signing_key.to_bytes(), Type::Ed25519)
            .unwrap();
        verify_request_signature(request);
    }

    #[test]
    fn sign_request_is_successful_secp256k1() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let request = create_order_request()
            .sign(signing_key.to_bytes(), Type::Secp256k1)
            .unwrap();
        verify_request_signature(request);
    }

    fn create_order_request() -> CreateOrderRequest {
        CreateOrderRequest {
            signed_fields: CreateOrderRequestSignedFields {
                symbol: "ETH-PERP".into(),
                account_address:
                    "0x8e78225d72b1d7b1f63e5e9f88f09b12ca66c84e2fc8b91fc10f6a0c51230615".into(),
                price_e9: "3500000000000".into(),
                quantity_e9: "100000000".into(),
                side: OrderSide::Long,
                leverage_e9: "0".into(),
                is_isolated: false,
                salt: "1725930601205".into(),
                ids_id: "0x8e78225d72b1d7b1f63e5e9f88f09b12ca66c84e2fc8b91fc10f6a0c51230615".into(),
                expires_at_utc_millis: 2037603360000,
                signed_at_utc_millis: 1725931543867,
            },
            ..CreateOrderRequest::default()
        }
    }
}
