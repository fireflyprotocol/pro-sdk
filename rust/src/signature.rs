use std::{borrow::Cow, fmt};

use blake2::digest::consts::U32;
use blake2::{Blake2b, Digest};
use serde::Serialize;
use sui_crypto::{ed25519::Ed25519PrivateKey, secp256k1::Secp256k1PrivateKey, SuiSigner};
use sui_sdk_types::{PersonalMessage, SignatureScheme};

pub type PrivateKey = [u8; 32];

#[derive(Debug)]
pub enum Error {
    Serialization,
    Signature(String),
    PrivateKey(String),
    PublicKeyRecoveryId,
    UnsupportedSignatureScheme(SignatureScheme),
    ParseNumber(String, String),
    Address(String),
    Hash,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Serialization => write!(f, "Error serializing request JSON"),
            Error::Signature(error) => write!(f, "Error signing request: {error}"),
            Error::PrivateKey(error) => write!(f, "Error creating private key: {error}"),
            Error::PublicKeyRecoveryId => {
                write!(f, "Invalid secp256k1 recovery ID")
            }
            Error::UnsupportedSignatureScheme(scheme) => {
                write!(f, "Unsupported signature scheme: {}", scheme.name())
            }
            Error::ParseNumber(field, input) => {
                write!(f, "Error parsing number: {field} = {input}")
            }
            Error::Address(address) => write!(f, "Error parsing address: {address}"),
            Error::Hash => write!(f, "Error hashing request"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
/// Extension methods for HTTP Requests that have to be signed.
pub trait RequestExt: Sized {
    /// Signs this request using the specified key
    ///
    /// # Errors
    ///
    /// Will return `Err` if the specified request cannot be serialized, or if the specified
    /// request can't be signed.
    fn sign(self, private_key: PrivateKey, scheme: SignatureScheme) -> Result<Self>;

    /// Computes the hash of the request.
    ///
    /// This is a unique identifier for the request.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the request cannot be serialized.
    fn compute_hash(self) -> Result<String>;
}

pub fn compute_hash<T: Serialize>(request: &T) -> Result<String> {
    let bcs = bcs::to_bytes(request).map_err(|_| Error::Hash)?;
    Ok(hex::encode(Blake2b::<U32>::digest(bcs)))
}

pub fn signature<T: Serialize>(
    request: T,
    private_key: PrivateKey,
    scheme: SignatureScheme,
) -> Result<String> {
    match scheme {
        SignatureScheme::Ed25519 => sign_ed25519(request, Ed25519PrivateKey::new(private_key)),
        SignatureScheme::Secp256k1 => sign_secp256k1(
            request,
            Secp256k1PrivateKey::new(private_key)
                .map_err(|err| Error::PrivateKey(err.to_string()))?,
        ),
        _ => Err(Error::UnsupportedSignatureScheme(scheme)),
    }
}

fn sign_ed25519<T: Serialize>(request: T, private_key: Ed25519PrivateKey) -> Result<String> {
    let serialized = serialize(request)?;
    let personal_message = PersonalMessage(Cow::Borrowed(serialized.as_bytes()));
    let signature = private_key
        .sign_personal_message(&personal_message)
        .map_err(|err| Error::Signature(err.to_string()))?;

    Ok(signature.to_base64())
}

fn sign_secp256k1<T: Serialize>(request: T, private_key: Secp256k1PrivateKey) -> Result<String> {
    let serialized = serialize(request)?;
    let personal_message = PersonalMessage(Cow::Borrowed(serialized.as_bytes()));
    let signature = private_key
        .sign_personal_message(&personal_message)
        .map_err(|err| Error::Signature(err.to_string()))?;

    Ok(signature.to_base64())
}

pub fn serialize<T: Serialize>(request: T) -> Result<String> {
    serde_json::to_string_pretty(&request).map_err(|_| Error::Serialization)
}

pub fn parse_u64(field: &str, input: &str) -> Result<u64> {
    let parsed = input
        .parse()
        .map_err(|_| Error::ParseNumber(field.to_string(), input.to_string()))?;
    Ok(parsed)
}

pub mod conversion {

    use serde::Serialize;
    use std::fmt::{Display, Formatter};

    pub mod signable {
        use super::{Display, Formatter, Serialize};

        pub enum ClientPayloadType {
            WithdrawRequest,
            OrderRequest,
            AuthorizeAccount,
            LeverageAdjustment,
            AdjustIsolatedMargin,
        }

        impl PartialEq for ClientPayloadType {
            fn eq(&self, other: &Self) -> bool {
                self.to_string() == other.to_string()
            }
        }

        impl Display for ClientPayloadType {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    ClientPayloadType::WithdrawRequest => write!(f, "Bluefin Pro Withdrawal"),
                    ClientPayloadType::OrderRequest => write!(f, "Bluefin Pro Order"),
                    ClientPayloadType::AuthorizeAccount => {
                        write!(f, "Bluefin Pro Authorize Account")
                    }
                    ClientPayloadType::LeverageAdjustment => {
                        write!(f, "Bluefin Pro Leverage Adjustment")
                    }
                    ClientPayloadType::AdjustIsolatedMargin => {
                        write!(f, "Bluefin Pro Margin Adjustment")
                    }
                }
            }
        }

        #[derive(Serialize)]
        enum PositionType {
            Isolated,
            Cross,
        }

        impl Display for PositionType {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    PositionType::Isolated => write!(f, "ISOLATED"),
                    PositionType::Cross => write!(f, "CROSS"),
                }
            }
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct WithdrawRequest {
            #[serde(rename = "type")]
            pub r#type: String,
            pub eds: String,
            pub asset_symbol: String,
            pub account: String,
            pub amount: String,
            pub salt: String,
            pub signed_at: String,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CreateOrderRequest {
            #[serde(rename = "type")]
            pub r#type: String,
            pub ids: String,
            pub account: String,
            pub market: String,
            pub price: String,
            pub quantity: String,
            pub leverage: String,
            pub side: String,          // [`SHORT`, `LONG`] string
            pub position_type: String, // [`PositionType`] enum as a string
            pub expiration: String,
            pub salt: String,
            pub signed_at: String,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct UpdateAccountPositionLeverageRequest {
            #[serde(rename = "type")]
            pub r#type: String,
            pub ids: String,
            pub account: String,
            pub market: String,
            pub leverage: String,
            pub salt: String,
            pub signed_at: String,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthorizeAccountRequest {
            #[serde(rename = "type")]
            pub r#type: String,
            pub ids: String,
            pub account: String,
            pub user: String,
            pub status: bool, // True when AUTHORIZE, false when DEAUTHORIZE
            pub salt: String,
            pub signed_at: String,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct DeauthorizeAccountRequest {
            #[serde(rename = "type")]
            pub r#type: String,
            pub ids: String,
            pub account: String,
            pub user: String,
            pub status: bool, // True when AUTHORIZE, false when DEAUTHORIZE
            pub salt: String,
            pub signed_at: String,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AdjustIsolatedMarginRequest {
            #[serde(rename = "type")]
            pub r#type: String,
            pub ids: String,
            pub account: String,
            pub market: String,
            pub add: bool,
            pub amount: String,
            pub salt: String,
            pub signed_at: String,
        }

        impl From<bluefin_api::models::WithdrawRequest> for WithdrawRequest {
            fn from(val: bluefin_api::models::WithdrawRequest) -> Self {
                WithdrawRequest {
                    r#type: ClientPayloadType::WithdrawRequest.to_string(),
                    eds: val.signed_fields.eds_id,
                    asset_symbol: val.signed_fields.asset_symbol,
                    account: val.signed_fields.account_address,
                    amount: val.signed_fields.amount_e9,
                    salt: val.signed_fields.salt,
                    signed_at: val.signed_fields.signed_at_millis.to_string(),
                }
            }
        }

        impl From<bluefin_api::models::CreateOrderRequest> for CreateOrderRequest {
            fn from(val: bluefin_api::models::CreateOrderRequest) -> Self {
                CreateOrderRequest {
                    r#type: ClientPayloadType::OrderRequest.to_string(),
                    ids: val.signed_fields.ids_id,
                    account: val.signed_fields.account_address,
                    market: val.signed_fields.symbol,
                    price: val.signed_fields.price_e9,
                    quantity: val.signed_fields.quantity_e9,
                    leverage: val.signed_fields.leverage_e9,
                    side: val.signed_fields.side.to_string(),
                    position_type: if val.signed_fields.is_isolated {
                        PositionType::Isolated.to_string()
                    } else {
                        PositionType::Cross.to_string()
                    },
                    expiration: val.signed_fields.expires_at_millis.to_string(),
                    salt: val.signed_fields.salt,
                    signed_at: val.signed_fields.signed_at_millis.to_string(),
                }
            }
        }

        impl From<bluefin_api::models::AccountPositionLeverageUpdateRequest>
            for UpdateAccountPositionLeverageRequest
        {
            fn from(val: bluefin_api::models::AccountPositionLeverageUpdateRequest) -> Self {
                UpdateAccountPositionLeverageRequest {
                    r#type: ClientPayloadType::LeverageAdjustment.to_string(),
                    ids: val.signed_fields.ids_id,
                    account: val.signed_fields.account_address,
                    market: val.signed_fields.symbol,
                    leverage: val.signed_fields.leverage_e9,
                    salt: val.signed_fields.salt,
                    signed_at: val.signed_fields.signed_at_millis.to_string(),
                }
            }
        }

        impl From<bluefin_api::models::AccountAuthorizationRequest> for AuthorizeAccountRequest {
            fn from(val: bluefin_api::models::AccountAuthorizationRequest) -> Self {
                AuthorizeAccountRequest {
                    r#type: ClientPayloadType::AuthorizeAccount.to_string(),
                    ids: val.signed_fields.ids_id,
                    account: val.signed_fields.account_address,
                    user: val.signed_fields.authorized_account_address,
                    status: true,
                    salt: val.signed_fields.salt,
                    signed_at: val.signed_fields.signed_at_millis.to_string(),
                }
            }
        }

        impl From<bluefin_api::models::AccountAuthorizationRequest> for DeauthorizeAccountRequest {
            fn from(val: bluefin_api::models::AccountAuthorizationRequest) -> Self {
                DeauthorizeAccountRequest {
                    r#type: ClientPayloadType::AuthorizeAccount.to_string(),
                    ids: val.signed_fields.ids_id,
                    account: val.signed_fields.account_address,
                    user: val.signed_fields.authorized_account_address,
                    status: false,
                    salt: val.signed_fields.salt,
                    signed_at: val.signed_fields.signed_at_millis.to_string(),
                }
            }
        }

        impl From<bluefin_api::models::AdjustIsolatedMarginRequest> for AdjustIsolatedMarginRequest {
            fn from(val: bluefin_api::models::AdjustIsolatedMarginRequest) -> Self {
                use bluefin_api::models::AdjustMarginOperation;

                AdjustIsolatedMarginRequest {
                    r#type: ClientPayloadType::AdjustIsolatedMargin.to_string(),
                    ids: val.signed_fields.ids_id,
                    account: val.signed_fields.account_address,
                    market: val.signed_fields.symbol,
                    add: val.signed_fields.operation == AdjustMarginOperation::Add,
                    amount: val.signed_fields.quantity_e9,
                    salt: val.signed_fields.salt,
                    signed_at: val.signed_fields.signed_at_millis.to_string(),
                }
            }
        }
    }

    pub mod hashable {
        use crate::signature::{self, parse_u64};

        use super::{hashable, Display, Formatter, Serialize};
        use serde::Deserialize;
        use sui_sdk_types::Address;

        #[derive(Serialize)]
        enum PositionType {
            Isolated,
            Cross,
        }

        impl Display for PositionType {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    PositionType::Isolated => write!(f, "ISOLATED"),
                    PositionType::Cross => write!(f, "CROSS"),
                }
            }
        }

        #[derive(Serialize, Deserialize)]
        pub struct WithdrawRequest {
            pub eds: Address,
            pub asset_symbol: String,
            pub account: Address,
            pub amount: u64,
            pub salt: u64,
            pub signed_at: u64,
        }

        #[derive(Serialize, Deserialize)]
        pub struct AuthorizeAccountRequest {
            pub ids: Address,
            pub account: Address,
            pub user: Address,
            pub status: bool,
            pub salt: u64,
            pub signed_at: u64,
        }

        #[derive(Serialize, Deserialize)]
        pub struct DeauthorizeAccountRequest {
            pub ids: Address,
            pub account: Address,
            pub user: Address,
            pub status: bool,
            pub salt: u64,
            pub signed_at: u64,
        }

        #[derive(Serialize, Deserialize)]
        pub struct CreateOrderRequest {
            pub ids: Address,
            pub account: Address,
            pub market: String,
            pub price: u64,
            pub quantity: u64,
            pub leverage: u64,
            pub side: String,
            pub position_type: String,
            pub expiration: u64,
            pub salt: u64,
            pub signed_at: u64,
        }

        #[derive(Serialize, Deserialize)]
        pub struct AdjustLeverageRequest {
            pub ids: Address,
            pub account: Address,
            pub market: String,
            pub leverage: u64,
            pub salt: u64,
            pub signed_at: u64,
        }

        #[derive(Serialize, Deserialize)]
        pub struct AdjustIsolatedMarginRequest {
            pub ids: Address,
            pub account: Address,
            pub market: String,
            pub add: bool,
            pub amount: u64,
            pub salt: u64,
            pub signed_at: u64,
        }

        impl TryFrom<bluefin_api::models::WithdrawRequest> for hashable::WithdrawRequest {
            type Error = signature::Error;

            fn try_from(val: bluefin_api::models::WithdrawRequest) -> Result<Self, Self::Error> {
                let eds = Address::from_hex(&val.signed_fields.eds_id)
                    .map_err(|_| signature::Error::Address(val.signed_fields.eds_id.to_string()))?;
                let asset_symbol = val.signed_fields.asset_symbol;
                let account =
                    Address::from_hex(&val.signed_fields.account_address).map_err(|_| {
                        signature::Error::Address(val.signed_fields.account_address.to_string())
                    })?;
                let amount = parse_u64("amount", &val.signed_fields.amount_e9)?;
                let salt = parse_u64("salt", &val.signed_fields.salt)?;
                let signed_at = val.signed_fields.signed_at_millis.unsigned_abs();

                Ok(WithdrawRequest {
                    eds,
                    asset_symbol,
                    account,
                    amount,
                    salt,
                    signed_at,
                })
            }
        }

        impl TryFrom<bluefin_api::models::AccountAuthorizationRequest>
            for hashable::AuthorizeAccountRequest
        {
            type Error = signature::Error;

            fn try_from(
                val: bluefin_api::models::AccountAuthorizationRequest,
            ) -> Result<Self, Self::Error> {
                let ids = Address::from_hex(&val.signed_fields.ids_id)
                    .map_err(|_| signature::Error::Address(val.signed_fields.ids_id.to_string()))?;
                let account =
                    Address::from_hex(&val.signed_fields.account_address).map_err(|_| {
                        signature::Error::Address(val.signed_fields.account_address.to_string())
                    })?;
                let user = Address::from_hex(&val.signed_fields.authorized_account_address)
                    .map_err(|_| {
                        signature::Error::Address(
                            val.signed_fields.authorized_account_address.to_string(),
                        )
                    })?;
                let status = true;
                let salt = parse_u64("salt", &val.signed_fields.salt)?;
                let signed_at = val.signed_fields.signed_at_millis.unsigned_abs();

                Ok(AuthorizeAccountRequest {
                    ids,
                    account,
                    user,
                    status,
                    salt,
                    signed_at,
                })
            }
        }

        impl TryFrom<bluefin_api::models::AccountAuthorizationRequest>
            for hashable::DeauthorizeAccountRequest
        {
            type Error = signature::Error;

            fn try_from(
                val: bluefin_api::models::AccountAuthorizationRequest,
            ) -> Result<Self, Self::Error> {
                let ids = Address::from_hex(&val.signed_fields.ids_id)
                    .map_err(|_| signature::Error::Address(val.signed_fields.ids_id.to_string()))?;
                let account =
                    Address::from_hex(&val.signed_fields.account_address).map_err(|_| {
                        signature::Error::Address(val.signed_fields.account_address.to_string())
                    })?;
                let user = Address::from_hex(&val.signed_fields.authorized_account_address)
                    .map_err(|_| {
                        signature::Error::Address(
                            val.signed_fields.authorized_account_address.to_string(),
                        )
                    })?;
                let status = false;
                let salt = parse_u64("salt", &val.signed_fields.salt)?;
                let signed_at = val.signed_fields.signed_at_millis.unsigned_abs();

                Ok(DeauthorizeAccountRequest {
                    ids,
                    account,
                    user,
                    status,
                    salt,
                    signed_at,
                })
            }
        }

        impl TryFrom<bluefin_api::models::CreateOrderRequest> for hashable::CreateOrderRequest {
            type Error = signature::Error;

            fn try_from(val: bluefin_api::models::CreateOrderRequest) -> Result<Self, Self::Error> {
                let ids = Address::from_hex(&val.signed_fields.ids_id)
                    .map_err(|_| signature::Error::Address(val.signed_fields.ids_id.to_string()))?;
                let account =
                    Address::from_hex(&val.signed_fields.account_address).map_err(|_| {
                        signature::Error::Address(val.signed_fields.account_address.to_string())
                    })?;
                let market = val.signed_fields.symbol;
                let price = parse_u64("price", &val.signed_fields.price_e9)?;
                let quantity = parse_u64("quantity", &val.signed_fields.quantity_e9)?;
                let leverage = parse_u64("leverage", &val.signed_fields.leverage_e9)?;
                let side = val.signed_fields.side.to_string();
                let position_type = if val.signed_fields.is_isolated {
                    PositionType::Isolated
                } else {
                    PositionType::Cross
                }
                .to_string();
                let expiration = val.signed_fields.expires_at_millis.unsigned_abs();
                let salt = parse_u64("salt", &val.signed_fields.salt)?;
                let signed_at = val.signed_fields.signed_at_millis.unsigned_abs();

                Ok(CreateOrderRequest {
                    ids,
                    account,
                    market,
                    price,
                    quantity,
                    leverage,
                    side,
                    position_type,
                    expiration,
                    salt,
                    signed_at,
                })
            }
        }

        impl TryFrom<bluefin_api::models::AccountPositionLeverageUpdateRequest>
            for hashable::AdjustLeverageRequest
        {
            type Error = signature::Error;

            fn try_from(
                val: bluefin_api::models::AccountPositionLeverageUpdateRequest,
            ) -> Result<Self, Self::Error> {
                let ids = Address::from_hex(&val.signed_fields.ids_id)
                    .map_err(|_| signature::Error::Address(val.signed_fields.ids_id.to_string()))?;
                let account =
                    Address::from_hex(&val.signed_fields.account_address).map_err(|_| {
                        signature::Error::Address(val.signed_fields.account_address.to_string())
                    })?;
                let market = val.signed_fields.symbol;
                let leverage = parse_u64("leverage", &val.signed_fields.leverage_e9)?;
                let salt = parse_u64("salt", &val.signed_fields.salt)?;
                let signed_at = val.signed_fields.signed_at_millis.unsigned_abs();

                Ok(AdjustLeverageRequest {
                    ids,
                    account,
                    market,
                    leverage,
                    salt,
                    signed_at,
                })
            }
        }

        impl TryFrom<bluefin_api::models::AdjustIsolatedMarginRequest>
            for hashable::AdjustIsolatedMarginRequest
        {
            type Error = signature::Error;

            fn try_from(
                val: bluefin_api::models::AdjustIsolatedMarginRequest,
            ) -> Result<Self, Self::Error> {
                use bluefin_api::models::AdjustMarginOperation;

                let ids = Address::from_hex(&val.signed_fields.ids_id)
                    .map_err(|_| signature::Error::Address(val.signed_fields.ids_id.to_string()))?;
                let account =
                    Address::from_hex(&val.signed_fields.account_address).map_err(|_| {
                        signature::Error::Address(val.signed_fields.account_address.to_string())
                    })?;
                let market = val.signed_fields.symbol;
                let add = val.signed_fields.operation == AdjustMarginOperation::Add;
                let amount = parse_u64("quantity", &val.signed_fields.quantity_e9)?;
                let salt = parse_u64("salt", &val.signed_fields.salt)?;
                let signed_at = val.signed_fields.signed_at_millis.unsigned_abs();

                Ok(AdjustIsolatedMarginRequest {
                    ids,
                    account,
                    market,
                    add,
                    amount,
                    salt,
                    signed_at,
                })
            }
        }
    }
}

#[cfg(test)]
pub mod testing {
    use std::borrow::Cow;

    use serde::Serialize;
    use sui_crypto::{ed25519::Ed25519Verifier, secp256k1::Secp256k1Verifier, SuiVerifier};
    use sui_sdk_types::{PersonalMessage, SimpleSignature, UserSignature};

    pub fn verify_signature<T: Serialize, F: Serialize>(
        signer_address: &str,
        signature: &str,
        payload: T,
        conversion_func: fn(T) -> F,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let signature =
            UserSignature::from_base64(signature).map_err(|_| "Error parsing signature")?;

        let converted = conversion_func(payload);

        let serialized = serde_json::to_string_pretty(&converted)
            .map_err(|_| "Error serializing request JSON")?;

        let personal_message = PersonalMessage(Cow::Borrowed(serialized.as_bytes()));

        match signature {
            UserSignature::Simple(SimpleSignature::Ed25519 { public_key, .. }) => {
                Ed25519Verifier::new()
                    .verify_personal_message(&personal_message, &signature)
                    .map_err(|_| "Error verifying ed25519 signature")?;

                assert_eq!(signer_address, public_key.derive_address().to_hex());
            }

            UserSignature::Simple(SimpleSignature::Secp256k1 { public_key, .. }) => {
                Secp256k1Verifier::new()
                    .verify_personal_message(&personal_message, &signature)
                    .map_err(|_| "Error verifying secp256k1 signature")?;

                assert_eq!(signer_address, public_key.derive_address().to_hex());
            }

            _ => Err("Unsupported signature type".to_string())?,
        }

        Ok(())
    }
}
