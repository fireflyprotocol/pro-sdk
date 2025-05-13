use crate::core::PrivateKey;
use crate::signature::{self, Result};
use bluefin_api::models::CreateOrderRequest;
use sui_sdk_types::SignatureScheme;

use crate::signature::RequestExt;

impl RequestExt for CreateOrderRequest {
    fn sign(self, private_key: PrivateKey, scheme: SignatureScheme) -> Result<Self> {
        let converted = signature::conversion::signable::CreateOrderRequest::from(self.clone());

        let signature = signature::signature(converted, private_key, scheme)?;
        Ok(CreateOrderRequest { signature, ..self })
    }

    fn compute_hash(self) -> Result<String> {
        let converted =
            signature::conversion::hashable::CreateOrderRequest::try_from(self.clone())?;
        signature::compute_hash(&converted)
    }
}

#[cfg(test)]
mod tests {
    use bluefin_api::models::{CreateOrderRequest, CreateOrderRequestSignedFields, OrderSide};
    use rand::rngs::OsRng;
    use sui_crypto::ed25519::Ed25519VerifyingKey;
    use sui_crypto::secp256k1::Secp256k1VerifyingKey;
    use sui_sdk_types::{Ed25519PublicKey, Secp256k1PublicKey, SignatureScheme};

    use crate::signature::RequestExt;
    use crate::signature::{self, testing::verify_signature};

    fn verify_request_signature(request: CreateOrderRequest, signer_address: &str) {
        assert!(!request.signature.is_empty());

        match verify_signature(
            signer_address,
            &request.signature.clone(),
            request.clone(),
            signature::conversion::signable::CreateOrderRequest::from,
        ) {
            Ok(()) => {}
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn sign_request_is_successful_ed25519() {
        let private_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
        let signer_address = Ed25519VerifyingKey::new(&Ed25519PublicKey::new(
            private_key.verifying_key().to_bytes(),
        ))
        .unwrap()
        .public_key()
        .derive_address()
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
        .derive_address()
        .to_hex();

        let request = create_order_request()
            .sign(private_key.secret_bytes(), SignatureScheme::Secp256k1)
            .unwrap();
        verify_request_signature(request, &signer_address);
    }

    #[test]
    fn compute_hash_is_successful() {
        let request = create_order_request();
        let hash = request.compute_hash().unwrap();
        assert!(!hash.is_empty());
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
                expires_at_millis: 2037603360000,
                signed_at_millis: 1725931543867,
            },
            ..CreateOrderRequest::default()
        }
    }
}
