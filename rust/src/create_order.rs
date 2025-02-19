use crate::core::PrivateKey;
use crate::signature::{self, Result};
use blake2::digest::consts::U32;
use blake2::{Blake2b, Digest};
use bluefin_api::models::CreateOrderRequest;
use sui_sdk_types::SignatureScheme;

use crate::signature::{serialize, RequestExt};

impl RequestExt for CreateOrderRequest {
    fn sign(self, private_key: PrivateKey, scheme: SignatureScheme) -> Result<Self> {
        let converted = signature::conversion::UICreateOrderRequest::from(self.clone());

        let signature = signature::signature(converted, private_key, scheme)?;
        let order_hash = hex::encode(Blake2b::<U32>::digest(
            serialize(&self.signed_fields).map_err(|_| signature::Error::Serialization)?,
        ));
        Ok(CreateOrderRequest {
            signature,
            order_hash,
            ..self
        })
    }
}

#[cfg(test)]
mod tests {
    use blake2::digest::consts::U32;
    use blake2::{Blake2b, Digest};
    use bluefin_api::models::{CreateOrderRequest, CreateOrderRequestSignedFields, OrderSide};
    use rand::rngs::OsRng;
    use sui_crypto::ed25519::Ed25519VerifyingKey;
    use sui_crypto::secp256k1::Secp256k1VerifyingKey;
    use sui_sdk_types::{Ed25519PublicKey, Secp256k1PublicKey, SignatureScheme};

    use crate::signature::RequestExt;
    use crate::signature::{self, testing::verify_signature};

    fn verify_request_signature(request: CreateOrderRequest, signer_address: &str) {
        assert!(!request.signature.is_empty());
        assert!(!request.order_hash.is_empty());

        match verify_signature(
            signer_address,
            &request.signature.clone(),
            request.clone(),
            signature::conversion::UICreateOrderRequest::from,
        ) {
            Ok(_) => {}
            Err(e) => panic!("{e}"),
        }

        assert_eq!(
            request.order_hash,
            hex::encode(Blake2b::<U32>::digest(
                serde_json::to_string_pretty(&request.signed_fields)
                    .unwrap()
                    .as_bytes(),
            ))
        );
    }

    #[test]
    fn sign_request_is_successful_ed25519() {
        let private_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
        let signer_address = Ed25519VerifyingKey::new(&Ed25519PublicKey::new(
            private_key.verifying_key().to_bytes(),
        ))
        .unwrap()
        .public_key()
        .to_address()
        .to_hex();
        let request = create_order_request()
            .sign(private_key.to_bytes(), SignatureScheme::Ed25519)
            .unwrap();
        verify_request_signature(request, &signer_address);
    }

    #[test]
    fn sign_request_is_successful_secp256k1() {
        let private_key = secp256k1::SecretKey::new(&mut OsRng);
        let secp = secp256k1::Secp256k1::new();
        let signer_address = Secp256k1VerifyingKey::new(&Secp256k1PublicKey::new(
            private_key.public_key(&secp).serialize(),
        ))
        .unwrap()
        .public_key()
        .to_address()
        .to_hex();

        let request = create_order_request()
            .sign(private_key.secret_bytes(), SignatureScheme::Secp256k1)
            .unwrap();
        verify_request_signature(request, &signer_address);
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
