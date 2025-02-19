use std::borrow::Cow;
use std::fmt;

use crate::core::PrivateKey;
use crate::env::auth::url;
use crate::env::Environment;
use crate::signature;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use bluefin_api::apis::auth_api::{auth_token_refresh_put, auth_v2_token_post};
use bluefin_api::apis::configuration::Configuration;
use bluefin_api::models::{LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse};
use secp256k1::Message;
use sui_crypto::ed25519::Ed25519PrivateKey;
use sui_crypto::SuiSigner;
use sui_sdk_types::{PersonalMessage, SignatureScheme};

#[derive(Debug)]
pub enum Error {
    AuthenticationRequestFailed(String),
    AuthenticationRequestSerializationFailed(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AuthenticationRequestFailed(error) => {
                write!(f, "Authentication request failed: {error}")
            }
            Error::AuthenticationRequestSerializationFailed(error) => {
                write!(f, "Authentication request serialization failed: {error}")
            }
        }
    }
}

impl std::error::Error for Error {}

type AuthenticationResult<T> = Result<T, Error>;

pub trait Authenticate {
    /// Authenticates using the provided authentication request and the signature.
    /// Returns the Authentication JWT.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the request fails.
    fn authenticate(
        self,
        signature: &str,
        environment: Environment,
    ) -> impl std::future::Future<Output = AuthenticationResult<LoginResponse>> + Send;
}

pub trait RequestExt: Sized {
    /// Generates a signature for this request.  The signature will contain the public key.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the specified private key is invalid.
    fn signature(
        &self,
        scheme: SignatureScheme,
        private_key: PrivateKey,
    ) -> signature::Result<String>;
}

pub trait Refresh {
    fn refresh(
        self,
        environment: Environment,
    ) -> impl std::future::Future<Output = AuthenticationResult<RefreshTokenResponse>> + Send;
}

impl RequestExt for LoginRequest {
    /// Generates a signature for this request.  The signature will contain the public key.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the specified private key is invalid.
    fn signature(
        &self,
        scheme: SignatureScheme,
        private_key: PrivateKey,
    ) -> signature::Result<String> {
        let bytes = serde_json::to_vec(self).map_err(|_| signature::Error::Serialization)?;

        let personal_message = PersonalMessage(Cow::Borrowed(bytes.as_slice()));

        match scheme {
            SignatureScheme::Ed25519 => {
                let private_key = Ed25519PrivateKey::new(private_key);

                let signature = private_key
                    .sign_personal_message(&personal_message)
                    .map_err(|e| signature::Error::Signature(e.to_string()))?;

                Ok(signature.to_base64())
            }
            SignatureScheme::Secp256k1 => {
                const RECOVERY_CODE: u8 = 31;

                let secp = secp256k1::Secp256k1::signing_only();
                let private_key = secp256k1::SecretKey::from_byte_array(&private_key)
                    .map_err(|error| signature::Error::PrivateKey(error.to_string()))?;

                let signature = secp.sign_ecdsa_recoverable(
                    &Message::from_digest(personal_message.signing_digest()),
                    &private_key,
                );

                let public_key = private_key.public_key(&secp256k1::Secp256k1::signing_only());

                let (recovery_id, signature) = signature.serialize_compact();

                let mut components = vec![SignatureScheme::Secp256k1 as u8];
                components.push(
                    RECOVERY_CODE
                        + u8::try_from(i32::from(recovery_id))
                            .map_err(|_| signature::Error::PublicKeyRecoveryId)?,
                );
                components.extend(signature);
                components.extend(public_key.serialize());

                Ok(BASE64_STANDARD.encode(&components))
            }
            _ => Err(signature::Error::UnsupportedSignatureScheme(scheme)),
        }
    }
}

impl Authenticate for LoginRequest {
    async fn authenticate(
        self,
        signature: &str,
        environment: Environment,
    ) -> AuthenticationResult<LoginResponse> {
        let base_url = url(environment);

        let mut configuration = Configuration::new();
        configuration.base_path = String::from(base_url);

        let response = auth_v2_token_post(&configuration, signature, self)
            .await
            .map_err(|error| Error::AuthenticationRequestFailed(error.to_string()))?;

        Ok(response)
    }
}

impl Refresh for RefreshTokenRequest {
    async fn refresh(self, environment: Environment) -> AuthenticationResult<RefreshTokenResponse> {
        let base_url = url(environment);

        let mut configuration = Configuration::new();
        configuration.base_path = String::from(base_url);

        let response = auth_token_refresh_put(&configuration, self)
            .await
            .map_err(|error| Error::AuthenticationRequestFailed(error.to_string()))?;

        Ok(response)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::env;

    use super::*;
    use base64::prelude::BASE64_STANDARD;
    use base64::Engine;
    use chrono::Utc;
    use rand::rngs::OsRng;
    use secp256k1::ecdsa::RecoveryId;
    use secp256k1::Message;
    use sui_crypto::{ed25519::Ed25519Verifier, SuiVerifier};
    use sui_sdk_types::{Ed25519PublicKey, Secp256k1PublicKey, SimpleSignature, UserSignature};

    fn verify_signature(
        encoded_signature: &str,
        login_payload: &LoginRequest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let signature = UserSignature::from_base64(encoded_signature)
            .map_err(|_| "Could not base64 decode signature".to_string())?;

        let bytes = serde_json::to_vec(login_payload)
            .map_err(|_| "Could not serialize auth request".to_string())?;

        let personal_message = PersonalMessage(Cow::Borrowed(bytes.as_slice()));

        match signature {
            UserSignature::Simple(SimpleSignature::Ed25519 { public_key, .. }) => {
                let sui_address = public_key.to_address().to_hex();

                Ed25519Verifier::new()
                    .verify_personal_message(&personal_message, &signature)
                    .map_err(|_| "Invalid signature".to_string())?;

                assert_eq!(sui_address, login_payload.account_address);
            }
            _ => Err("Unsupported signature scheme".to_string())?,
        };

        Ok(())
    }

    #[test]
    fn sign_auth_request() -> Result<(), Box<dyn std::error::Error>> {
        // ed25519
        let private_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
        let public_key = private_key.verifying_key().to_bytes();
        let public_key = Ed25519PublicKey::new(public_key);

        let sui_address = public_key.to_address().to_hex();

        let auth_request = LoginRequest {
            account_address: sui_address,
            audience: env::auth::testnet::AUDIENCE.into(),
            signed_at_utc_millis: Utc::now().timestamp_millis(),
        };

        let signature = auth_request
            .signature(SignatureScheme::Ed25519, private_key.to_bytes())
            .map_err(|error| format!("{error:?}"))?;
        verify_signature(&signature, &auth_request)?;

        // secp256k1
        fn verify_secp256k1_signature(
            encoded_signature: &str,
            login_payload: &LoginRequest,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let signature_bytes = BASE64_STANDARD
                .decode(encoded_signature)
                .map_err(|_| "Could not base64 decode signature".to_string())?;
            // 27 is an old magic number inherited from Bitcoin and is used as a "magic constant"
            // 4 is a number used to indicate that the public key is compressed
            const RECOVERY_CODE: u8 = 31;
            // Extract the signature and public key bytes
            if signature_bytes.len() != 99 {
                // 1 signature type flag byte + 65 signature (1 recovery byte + 64 signature bytes) + 33 public key
                return Err("Invalid signature length".into());
            }
            let recovery_bit = signature_bytes[1] - RECOVERY_CODE;

            let signature = secp256k1::ecdsa::RecoverableSignature::from_compact(
                &signature_bytes[2..signature_bytes.len() - 33],
                RecoveryId::try_from(i32::from(recovery_bit))
                    .map_err(|_| "Invalid secp256k1 recovery ID".to_string())?,
            )?;
            let public_key = secp256k1::PublicKey::from_slice(&signature_bytes[(1 + 65)..])?;

            // get message bytes
            let bytes = serde_json::to_vec(login_payload)
                .map_err(|_| "Could not serialize auth request".to_string())?;
            let personal_message = PersonalMessage(Cow::Borrowed(bytes.as_slice()));

            // Verify the signature
            let message = Message::from_digest(personal_message.signing_digest());

            let recovered_public_key = signature
                .recover(&message)
                .map_err(|_| "Invalid secp256k1 signature".to_string())?;

            assert_eq!(public_key, recovered_public_key);
            Ok(())
        }

        let signature = auth_request
            .signature(SignatureScheme::Secp256k1, private_key.to_bytes())
            .map_err(|error| format!("{error:?}"))?;
        verify_secp256k1_signature(&signature, &auth_request)?;

        Ok(())
    }

    #[tokio::test]
    async fn authenticate_staging_ed25519() -> Result<(), Box<dyn std::error::Error>> {
        let private_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
        let public_key = private_key.verifying_key().to_bytes();
        let public_key = Ed25519PublicKey::new(public_key);

        let sui_address = public_key.to_address().to_hex();

        let auth_request = LoginRequest {
            account_address: sui_address,
            audience: env::auth::testnet::AUDIENCE.into(),
            signed_at_utc_millis: Utc::now().timestamp_millis(),
        };

        let signature = auth_request
            .signature(SignatureScheme::Ed25519, private_key.to_bytes())
            .map_err(|error| format!("{error:?}"))?;

        auth_request
            .authenticate(&signature, Environment::Testnet)
            .await
            .map_err(|error| format!("{error:?}"))?;
        Ok(())
    }

    #[tokio::test]
    async fn authenticate_staging_secp256k1() -> Result<(), Box<dyn std::error::Error>> {
        let (private_key, public_key) = secp256k1::generate_keypair(&mut OsRng);
        let public_key = Secp256k1PublicKey::new(public_key.serialize());

        let sui_address = public_key.to_address().to_hex();

        let auth_request = LoginRequest {
            account_address: sui_address,
            audience: env::auth::testnet::AUDIENCE.into(),
            signed_at_utc_millis: Utc::now().timestamp_millis(),
        };

        let signature = auth_request
            .signature(SignatureScheme::Secp256k1, private_key.secret_bytes())
            .map_err(|error| format!("{error:?}"))?;

        auth_request
            .authenticate(&signature, Environment::Testnet)
            .await
            .map_err(|error| format!("{error:?}"))?;
        Ok(())
    }
}
