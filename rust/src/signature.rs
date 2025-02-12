use blake2::digest::consts::U32;
use blake2::Digest;
use ed25519_dalek::Signer;
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    PrivateKeyLength(usize),
    PrivateKey(String),
    EncodedObject(String),
    SuiAddress(String),
    RequestField(String),
    Components(String),
    RequestObject(String),
    AuthSignatureHashOutput(String),
    PublicKeyRecoveryId(String),
    InvalidSignature(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Define user-friendlier error messages.
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Components {
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>, // raw bytes
    pub signature_type: u8,
}

impl Components {
    pub fn encode(&self) -> Result<String> {
        let bcs_encoded =
            bcs::to_bytes(self).map_err(|error| Error::Components(error.to_string()))?;
        Ok(hex::encode(&bcs_encoded))
    }
}

pub struct ParseError<T: Sized> {
    field: &'static str,
    _type: PhantomData<T>,
}

impl<T: Sized> ParseError<T> {
    fn new(field: &'static str) -> ParseError<T> {
        ParseError {
            field,
            _type: PhantomData,
        }
    }
}

impl<T: Sized> From<ParseError<T>> for Error {
    fn from(value: ParseError<T>) -> Self {
        Error::RequestField(format!(
            "failed to parse {} as {}",
            value.field,
            type_name::<T>()
        ))
    }
}

pub fn parse<T: FromStr>(s: &str, field: &'static str) -> std::result::Result<T, ParseError<T>> {
    s.parse().map_err(|_| ParseError::<T>::new(field))
}

/// TODO: Replace with tuple struct.
pub type PrivateKey = [u8; 32];
pub type Ed25519PublicKey = [u8; 32];
pub type Secp256k1PublicKey = [u8; 33]; //Compressed only

#[derive(Clone)]
pub enum Type {
    Secp256k1,
    Ed25519,
}

impl From<Type> for u8 {
    fn from(value: Type) -> Self {
        match value {
            Type::Ed25519 => 0,
            Type::Secp256k1 => 1,
        }
    }
}

pub type SuiAddress = String;

pub trait IntoSuiAddress {
    fn into_sui_address(self) -> SuiAddress;
}

impl IntoSuiAddress for Ed25519PublicKey {
    fn into_sui_address(self) -> SuiAddress {
        let mut components = vec![u8::from(Type::Ed25519)];
        components.extend(self);
        encode_address(components)
    }
}

impl IntoSuiAddress for Secp256k1PublicKey {
    fn into_sui_address(self) -> SuiAddress {
        let mut components = vec![u8::from(Type::Secp256k1)];
        components.extend(self);
        encode_address(components)
    }
}

fn encode_address(components: Vec<u8>) -> String {
    let hashed = blake2::Blake2b::<U32>::digest(components);

    //Normalize
    let hex_encoded_address = hex::encode(hashed).to_lowercase();
    let stripped_address = hex_encoded_address
        .strip_prefix("0x")
        .unwrap_or(&hex_encoded_address);
    format!("0x{:0>width$}", stripped_address, width = 64)
}

/// Extension methods for HTTP Requests that have to be signed.
pub trait RequestExt: Sized {
    /// Signs this request using the specified key
    ///
    /// # Errors
    ///
    /// Will return `Err` if the specified request cannot be serialized, or if the specified private
    /// key is invalid.
    fn sign(self, private_key: PrivateKey, type_id: Type) -> Result<Self>;
}

/// Signs the given SHA-encoded request bytes using the specified private key and signature type.
/// Supports both Ed25519 and Secp256k1 signature types.
///
/// # Errors
///
/// Will return `Err(Error::PrivateKeyLength)` if the private key length is invalid for the
/// Secp256k1 signature type, or `Err(Error::EncodedObject)` if the SHA256 encoding is invalid for
/// message creation.
pub fn sign(
    private_key: PrivateKey,
    type_id: Type,
    sha_encoded_request_bytes: &[u8],
) -> Result<String> {
    let (payload_signature, public_key) = match type_id {
        Type::Ed25519 => {
            let private_key_bytes = ed25519_dalek::SecretKey::from(private_key);
            let signing_key = ed25519_dalek::SigningKey::from_bytes(&private_key_bytes);
            (
                signing_key.sign(sha_encoded_request_bytes).to_vec(),
                signing_key.verifying_key().to_bytes().to_vec(),
            )
        }
        Type::Secp256k1 => {
            let secp = secp256k1::Secp256k1::signing_only();
            let private_key = secp256k1::SecretKey::from_byte_array(&private_key)
                .map_err(|_| Error::PrivateKeyLength(private_key.len()))?;
            let message = secp256k1::Message::from_digest(
                sha_encoded_request_bytes
                    .try_into()
                    .map_err(|_| Error::EncodedObject("Invalid Sha256 encoding".to_string()))?,
            );
            (
                secp.sign_ecdsa(&message, &private_key)
                    .serialize_compact()
                    .to_vec(),
                private_key.public_key(&secp).serialize().to_vec(),
            )
        }
    };

    Components {
        signature: payload_signature,
        public_key,
        signature_type: type_id.into(),
    }
    .encode()
}

#[cfg(test)]
pub mod testing {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};
    use secp256k1::ecdsa::Signature as SecpSignature;
    use secp256k1::{Message, PublicKey, Secp256k1};
    use sha2::{Digest, Sha256};

    use super::*;

    type Result<T> = std::result::Result<T, &'static str>;

    fn read_components(signature: &str) -> Result<Components> {
        let hex_decoded = hex::decode(signature)
            .map_err(|_| "Decoding Signature Components Hex String failed")?;
        bcs::from_bytes(&hex_decoded).map_err(|_| "Creating Signature Components failed")
    }

    fn verify_ed25519_signature(
        bytes: &[u8],
        public_key: &[u8],
        bcs_encoded_payload: &[u8],
    ) -> Result<()> {
        let bytes: &[u8; 64] = bytes
            .try_into()
            .map_err(|_| "Signature has to be 64 bytes.")?;

        let public_key_bytes_32: &[u8; 32] = public_key
            .try_into()
            .map_err(|_| "Public key has to be 32 bytes.")?;

        let signature = Signature::from_bytes(bytes);

        let verification_key = VerifyingKey::from_bytes(public_key_bytes_32)
            .map_err(|_| "Creating ed25519 Public Key from public key bytes failed")?;

        verification_key
            .verify(bcs_encoded_payload, &signature)
            .map_err(|_| "ed25519 Signature Verification failed")
    }

    fn verify_secp256_signature(
        bytes: &[u8],
        public_key: &[u8],
        bcs_encoded_payload: &[u8],
    ) -> Result<()> {
        let payload_digest: [u8; 32] = bcs_encoded_payload
            .try_into()
            .map_err(|_| "secp256k1 payload digest has to be 32 bytes.")?;

        let secp = Secp256k1::verification_only();

        let message = Message::from_digest(payload_digest);
        let public_key = PublicKey::from_slice(public_key)
            .map_err(|_| "Creating secp256k1 Public Key failed")?;

        let signature = SecpSignature::from_compact(bytes)
            .map_err(|_| "Creating secp256k1 Signature failed")?;

        secp.verify_ecdsa(&message, &signature, &public_key)
            .map_err(|_| "secp256k1 Signature Verification failed")
    }

    pub fn verify_signature<T: Serialize, F: Serialize>(
        signature: &str,
        payload: T,
        bcs_conversion_func: fn(T) -> super::Result<F>,
    ) -> Result<()> {
        let components = read_components(signature)?;

        let bcs_dto_request =
            bcs_conversion_func(payload).map_err(|_| "Failed to convert Request to BCS DTO")?;
        let serialized_payload =
            bcs::to_bytes(&bcs_dto_request).map_err(|_| "BCS encoding failed")?;

        let sha_encoded_payload = Sha256::digest(&serialized_payload);

        match components.signature_type {
            0 => verify_ed25519_signature(
                &components.signature,
                &components.public_key,
                &sha_encoded_payload,
            ),
            1 => verify_secp256_signature(
                &components.signature,
                &components.public_key,
                &sha_encoded_payload,
            ),
            _ => Err("Invalid Signature Type"),
        }
    }
}
