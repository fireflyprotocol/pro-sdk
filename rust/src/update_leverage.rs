use crate::core::PrivateKey;
use crate::signature::{self, Result};
use bluefin_api::models::AccountPositionLeverageUpdateRequest;
use sui_sdk_types::SignatureScheme;

use crate::signature::RequestExt;

impl RequestExt for AccountPositionLeverageUpdateRequest {
    fn sign(self, private_key: PrivateKey, scheme: SignatureScheme) -> Result<Self> {
        let converted =
            signature::conversion::UIUpdateAccountPositionLeverageRequest::from(self.clone());

        let signature = signature::signature(converted, private_key, scheme)?;
        Ok(AccountPositionLeverageUpdateRequest { signature, ..self })
    }
}

#[cfg(test)]
mod tests {
    use bluefin_api::models::{
        AccountPositionLeverageUpdateRequest, AccountPositionLeverageUpdateRequestSignedFields,
    };
    use rand::rngs::OsRng;
    use sui_crypto::ed25519::Ed25519VerifyingKey;
    use sui_crypto::secp256k1::Secp256k1VerifyingKey;
    use sui_sdk_types::{Ed25519PublicKey, Secp256k1PublicKey, SignatureScheme};

    use crate::signature::RequestExt;
    use crate::signature::{self, testing::verify_signature};

    fn verify_request_signature(
        request: AccountPositionLeverageUpdateRequest,
        signer_address: &str,
    ) {
        assert!(!request.signature.is_empty());

        match verify_signature(
            signer_address,
            &request.signature.clone(),
            request.clone(),
            signature::conversion::UIUpdateAccountPositionLeverageRequest::from,
        ) {
            Ok(_) => {}
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
        .to_address()
        .to_hex();
        let request = update_leverage_request()
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

        let request = update_leverage_request()
            .sign(private_key.secret_bytes(), SignatureScheme::Secp256k1)
            .unwrap();
        verify_request_signature(request, &signer_address);
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
                signed_at_millis: 1725931543867,
            },
            ..AccountPositionLeverageUpdateRequest::default()
        }
    }
}
