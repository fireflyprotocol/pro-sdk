use bluefin_api::models::AccountAuthorizationRequest;
use sui_sdk_types::SignatureScheme;

use crate::{
    core::PrivateKey,
    signature::{self, RequestExt, Result},
};

pub struct AccountAuthorizationRequestExt {
    pub request: AccountAuthorizationRequest,
    pub is_authorize: bool,
}

impl AccountAuthorizationRequestExt {
    pub fn new(request: AccountAuthorizationRequest, is_authorize: bool) -> Self {
        Self {
            request,
            is_authorize,
        }
    }
}
impl RequestExt for AccountAuthorizationRequestExt {
    fn sign(self, private_key: PrivateKey, scheme: SignatureScheme) -> Result<Self> {
        let signature = if self.is_authorize {
            signature::signature(
                signature::conversion::UIAuthorizeAccountRequest::from(self.request.clone()),
                private_key,
                scheme,
            )?
        } else {
            signature::signature(
                signature::conversion::UIDeauthorizeAccountRequest::from(self.request.clone()),
                private_key,
                scheme,
            )?
        };

        Ok(AccountAuthorizationRequestExt {
            request: AccountAuthorizationRequest {
                signature,
                ..self.request
            },
            is_authorize: self.is_authorize,
        })
    }
}

#[cfg(test)]
mod tests {
    use bluefin_api::models::AccountAuthorizationRequestSignedFields;
    use rand::rngs::OsRng;
    use sui_crypto::{ed25519::Ed25519VerifyingKey, secp256k1::Secp256k1VerifyingKey};
    use sui_sdk_types::{Ed25519PublicKey, Secp256k1PublicKey};

    use crate::signature::testing::verify_signature;

    use super::*;

    fn verify_request_signature(request: AccountAuthorizationRequestExt, signer_address: &str) {
        assert!(!request.request.signature.is_empty());

        let result = if request.is_authorize {
            verify_signature(
                signer_address,
                &request.request.signature.clone(),
                request.request.clone(),
                signature::conversion::UIAuthorizeAccountRequest::from,
            )
        } else {
            verify_signature(
                signer_address,
                &request.request.signature.clone(),
                request.request.clone(),
                signature::conversion::UIDeauthorizeAccountRequest::from,
            )
        };

        match result {
            Ok(_) => {}
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn sign_authorize_request_is_successful_ed25519() {
        let private_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
        let signer_address = Ed25519VerifyingKey::new(&Ed25519PublicKey::new(
            private_key.verifying_key().to_bytes(),
        ))
        .unwrap()
        .public_key()
        .to_address()
        .to_hex();
        let request = account_authorization_request(true)
            .sign(private_key.to_bytes(), SignatureScheme::Ed25519)
            .unwrap();
        verify_request_signature(request, &signer_address);
    }

    #[test]
    fn sign_authorize_request_is_successful_secp256k1() {
        let private_key = secp256k1::SecretKey::new(&mut OsRng);
        let secp = secp256k1::Secp256k1::new();
        let signer_address = Secp256k1VerifyingKey::new(&Secp256k1PublicKey::new(
            private_key.public_key(&secp).serialize(),
        ))
        .unwrap()
        .public_key()
        .to_address()
        .to_hex();

        let request = account_authorization_request(true)
            .sign(private_key.secret_bytes(), SignatureScheme::Secp256k1)
            .unwrap();
        verify_request_signature(request, &signer_address);
    }

    #[test]
    fn sign_deauthorize_request_is_successful_ed25519() {
        let private_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
        let signer_address = Ed25519VerifyingKey::new(&Ed25519PublicKey::new(
            private_key.verifying_key().to_bytes(),
        ))
        .unwrap()
        .public_key()
        .to_address()
        .to_hex();
        let request = account_authorization_request(false)
            .sign(private_key.to_bytes(), SignatureScheme::Ed25519)
            .unwrap();
        verify_request_signature(request, &signer_address);
    }

    #[test]
    fn sign_deauthorize_request_is_successful_secp256k1() {
        let private_key = secp256k1::SecretKey::new(&mut OsRng);
        let secp = secp256k1::Secp256k1::new();
        let signer_address = Secp256k1VerifyingKey::new(&Secp256k1PublicKey::new(
            private_key.public_key(&secp).serialize(),
        ))
        .unwrap()
        .public_key()
        .to_address()
        .to_hex();

        let request = account_authorization_request(false)
            .sign(private_key.secret_bytes(), SignatureScheme::Secp256k1)
            .unwrap();
        verify_request_signature(request, &signer_address);
    }

    fn account_authorization_request(is_authorize: bool) -> AccountAuthorizationRequestExt {
        AccountAuthorizationRequestExt::new(
            AccountAuthorizationRequest {
                signed_fields: AccountAuthorizationRequestSignedFields {
                    account_address:
                        "0x8e78225d72b1d7b1f63e5e9f88f09b12ca66c84e2fc8b91fc10f6a0c51230615".into(),
                    authorized_account_address:
                        "0x8e78225d72b1d7b1f63e5e9f88f09b12ca66c84e2fc8b91fc10f6a0c51230615".into(),
                    ids_id: "0x8e78225d72b1d7b1f63e5e9f88f09b12ca66c84e2fc8b91fc10f6a0c51230615"
                        .into(),
                    salt: "1725930601205".into(),
                    signed_at_millis: 1725931543867,
                },
                ..AccountAuthorizationRequest::default()
            },
            is_authorize,
        )
    }
}
