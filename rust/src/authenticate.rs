use std::fmt;

use crate::env::Environment;
use crate::signature::{PrivateKey, Type};
use crate::{env, signature};
use base64::prelude::*;
use blake2::digest::consts::U32;
use blake2::Digest;
use bluefin_api::apis::auth_api::{auth_token_post, auth_token_refresh_put};
use bluefin_api::apis::configuration::Configuration;
use bluefin_api::models::{LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse};
use ed25519_dalek::Signer;
use secp256k1::Message;

#[derive(Debug)]
pub enum Error {
    AuthenticationRequestFailed(String),
    AuthenticationRequestSerializationFailed(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Define user-friendlier error messages.
        write!(f, "{self:?}")
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

pub trait Refresh {
    fn refresh(
        self,
        environment: Environment,
    ) -> impl std::future::Future<Output = AuthenticationResult<RefreshTokenResponse>> + Send;
}

pub trait RequestExt: Sized {
    /// Generates a signature for this request.  The signature will contain the public key.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the specified private key is invalid.
    fn signature(&self, signature_type: Type, private_key: PrivateKey)
        -> signature::Result<String>;
}

impl RequestExt for LoginRequest {
    /// Generates a signature for this request.  The signature will contain the public key.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the specified private key is invalid.
    fn signature(
        &self,
        signature_type: Type,
        private_key: PrivateKey,
    ) -> signature::Result<String> {
        let hash = blake2::Blake2b::<U32>::digest(
            &serde_json::to_vec(&self)
                .map_err(|error| signature::Error::RequestObject(error.to_string()))?,
        );

        let mut components = vec![u8::from(signature_type.clone())];

        match signature_type {
            Type::Ed25519 => {
                let private_key = ed25519_dalek::SigningKey::from_bytes(&private_key);
                let signature = private_key.sign(&hash);
                let public_key = private_key.verifying_key();

                components.extend(signature.to_bytes());
                components.extend(public_key.to_bytes());
            }
            Type::Secp256k1 => {
                const RECOVERY_CODE: u8 = 27 + 4;

                let secp = secp256k1::Secp256k1::signing_only();
                let private_key = secp256k1::SecretKey::from_byte_array(&private_key)
                    .map_err(|error| signature::Error::PrivateKey(error.to_string()))?;

                let signature = secp.sign_ecdsa_recoverable(
                    &Message::from_digest(
                        <[u8; 32]>::try_from(hash.as_slice())
                            .map_err(|error| signature::Error::EncodedObject(error.to_string()))?,
                    ),
                    &private_key,
                );

                let public_key = private_key.public_key(&secp256k1::Secp256k1::signing_only());

                let (recovery_id, signature) = signature.serialize_compact();

                components.push(
                    RECOVERY_CODE
                        + u8::try_from(i32::from(recovery_id)).map_err(|_| {
                            signature::Error::PublicKeyRecoveryId(
                                "Invalid secp256k1 recovery ID".into(),
                            )
                        })?,
                );
                components.extend(signature);
                components.extend(public_key.serialize());
            }
        }
        Ok(BASE64_URL_SAFE.encode(&components))
    }
}

impl Authenticate for LoginRequest {
    async fn authenticate(
        self,
        signature: &str,
        environment: Environment,
    ) -> AuthenticationResult<LoginResponse> {
        let base_url = match environment {
            Environment::Devnet => env::auth::devnet::URL,
            Environment::Testnet => env::auth::testnet::URL,
            Environment::Mainnet => env::auth::mainnet::URL,
        };

        let mut configuration = Configuration::new();
        configuration.base_path = String::from(base_url);

        let response = auth_token_post(&configuration, signature, self)
            .await
            .map_err(|error| Error::AuthenticationRequestFailed(error.to_string()))?;

        Ok(response)
    }
}

impl Refresh for RefreshTokenRequest {
    async fn refresh(self, environment: Environment) -> AuthenticationResult<RefreshTokenResponse> {
        let base_url = match environment {
            Environment::Devnet => env::auth::devnet::URL,
            Environment::Testnet => env::auth::testnet::URL,
            Environment::Mainnet => env::auth::mainnet::URL,
        };

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
    use super::*;
    use crate::signature::{Ed25519PublicKey, IntoSuiAddress, Secp256k1PublicKey};
    use chrono::Utc;
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};
    use rand::rngs::OsRng;
    use secp256k1::ecdsa::RecoveryId;

    #[test]
    fn sign_auth_request() -> Result<(), Box<dyn std::error::Error>> {
        fn verify_signature(
            signature_type: Type,
            encoded_signature: &str,
            login_payload: &LoginRequest,
        ) -> Result<(), Box<dyn std::error::Error>> {
            // Decode the Base64-encoded signature
            let signature_bytes = BASE64_URL_SAFE
                .decode(encoded_signature)
                .map_err(|_| "Could not base64 decode signature".to_string())?;

            match signature_type {
                Type::Ed25519 => {
                    // Extract the signature and public key bytes
                    if signature_bytes.len() != 97 {
                        // 1 signature type flag byte + 64 signature + 32 public key
                        return Err("Invalid signature length".into());
                    }
                    let signature =
                        <&[u8; 64]>::try_from(&signature_bytes[1..signature_bytes.len() - 32])
                            .map_err(|_| "Invalid signature length".to_string())?;
                    let pub_key_bytes: [u8; 32] =
                        <[u8; 32]>::try_from(&signature_bytes[(1 + signature.len())..])
                            .map_err(|_| "Invalid public key".to_string())?;

                    // get message bytes
                    let message = serde_json::to_vec(login_payload)
                        .map_err(|_| "Could not serialize auth request".to_string())?;

                    // Verify the signature
                    let signature = Signature::from_bytes(signature);
                    let pub_key = VerifyingKey::from_bytes(&pub_key_bytes)?;

                    let hash = blake2::Blake2b::<U32>::digest(&message);
                    pub_key
                        .verify(&hash, &signature)
                        .map_err(|_| "Invalid signature".to_string())?;
                }
                Type::Secp256k1 => {
                    // 27 is an old magic number inherited from Bitcoin and is used as a "magic constant"
                    // 4 is a number used to indicate that the public key is compressed
                    const RECOVERY_CODE: u8 = 27 + 4;
                    // Extract the signature and public key bytes
                    if signature_bytes.len() != 99 {
                        // 1 signature type flag byte + 65 signature (1 recovery byte + 64 signature bytes) + 33 public key
                        return Err("Invalid signature length".into());
                    }
                    let recovery_bit = signature_bytes[1] - RECOVERY_CODE;

                    let signature = secp256k1::ecdsa::RecoverableSignature::from_compact(
                        &signature_bytes[2..signature_bytes.len() - 33],
                        RecoveryId::try_from(i32::from(recovery_bit)).map_err(|_| {
                            signature::Error::PublicKeyRecoveryId("Invalid Recovery ID".into())
                        })?,
                    )?;
                    let public_key =
                        secp256k1::PublicKey::from_slice(&signature_bytes[(1 + 65)..])?;

                    // get message bytes
                    let message = serde_json::to_vec(login_payload)
                        .map_err(|_| "Could not serialize auth request".to_string())?;

                    // Verify the signature
                    let message = Message::from_digest(<[u8; 32]>::try_from(
                        blake2::Blake2b::<U32>::digest(&message).as_slice(),
                    )?);

                    let recovered_public_key = signature
                        .recover(&message)
                        .map_err(|error| signature::Error::InvalidSignature(error.to_string()))?;

                    assert_eq!(public_key, recovered_public_key);
                }
            }

            Ok(())
        }

        // ed25519
        let private_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
        let public_key = Ed25519PublicKey::from(private_key.verifying_key().to_bytes());
        let sui_address = public_key.into_sui_address();

        let request = LoginRequest {
            account_address: sui_address,
            signed_at_utc_millis: Utc::now().timestamp_millis(),
            audience: env::auth::testnet::AUDIENCE.to_string(),
        };

        let signature = request
            .signature(Type::Ed25519, private_key.to_bytes())
            .map_err(|error| format!("{error:?}"))?;
        verify_signature(Type::Ed25519, &signature, &request)?;

        // secp256k1
        let signature = request.signature(Type::Secp256k1, private_key.to_bytes())?;
        verify_signature(Type::Secp256k1, &signature, &request)?;

        Ok(())
    }

    #[tokio::test]
    async fn authenticate_staging_ed25519() -> Result<(), Box<dyn std::error::Error>> {
        let private_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
        let public_key = private_key.verifying_key();
        let public_key = Ed25519PublicKey::from(public_key.to_bytes());

        let sui_address = public_key.into_sui_address();

        let auth_request = LoginRequest {
            account_address: sui_address,
            audience: env::auth::testnet::AUDIENCE.into(),
            signed_at_utc_millis: Utc::now().timestamp_millis(),
        };

        let signature = auth_request
            .signature(Type::Ed25519, private_key.to_bytes())
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
        let public_key = Secp256k1PublicKey::from(public_key.serialize());

        let sui_address = public_key.into_sui_address();

        let auth_request = LoginRequest {
            account_address: sui_address,
            audience: env::auth::testnet::AUDIENCE.into(),
            signed_at_utc_millis: Utc::now().timestamp_millis(),
        };

        let signature = auth_request
            .signature(Type::Secp256k1, private_key.secret_bytes())
            .map_err(|error| format!("{error:?}"))?;

        auth_request
            .authenticate(&signature, Environment::Testnet)
            .await
            .map_err(|error| format!("{error:?}"))?;
        Ok(())
    }
}
