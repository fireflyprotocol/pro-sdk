use std::{borrow::Cow, fmt};

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
    /// Will return `Err` if the specified request cannot be serialized, or if the specified request can't be signed.
    fn sign(self, private_key: PrivateKey, scheme: SignatureScheme) -> Result<Self>;
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

pub mod conversion {
    use bluefin_api::models::{
        AccountAuthorizationRequest, AccountPositionLeverageUpdateRequest, CreateOrderRequest,
        WithdrawRequest,
    };
    use serde::Serialize;
    use std::fmt::{Display, Formatter};

    pub enum ClientPayloadType {
        WithdrawRequest,
        OrderRequest,
        AuthorizeAccount,
        LeverageAdjustment,
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
                ClientPayloadType::AuthorizeAccount => write!(f, "Bluefin Pro Authorize Account"),
                ClientPayloadType::LeverageAdjustment => {
                    write!(f, "Bluefin Pro Leverage Adjustment")
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
    pub struct UIWithdrawRequest {
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
    pub struct UICreateOrderRequest {
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
    pub struct UIUpdateAccountPositionLeverageRequest {
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
    pub struct UIAuthorizeAccountRequest {
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
    pub struct UIDeauthorizeAccountRequest {
        #[serde(rename = "type")]
        pub r#type: String,
        pub ids: String,
        pub account: String,
        pub user: String,
        pub status: bool, // True when AUTHORIZE, false when DEAUTHORIZE
        pub salt: String,
        pub signed_at: String,
    }

    impl From<WithdrawRequest> for UIWithdrawRequest {
        fn from(val: WithdrawRequest) -> Self {
            UIWithdrawRequest {
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

    impl From<CreateOrderRequest> for UICreateOrderRequest {
        fn from(val: CreateOrderRequest) -> Self {
            UICreateOrderRequest {
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

    impl From<AccountPositionLeverageUpdateRequest> for UIUpdateAccountPositionLeverageRequest {
        fn from(val: AccountPositionLeverageUpdateRequest) -> Self {
            UIUpdateAccountPositionLeverageRequest {
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

    impl From<AccountAuthorizationRequest> for UIAuthorizeAccountRequest {
        fn from(val: AccountAuthorizationRequest) -> Self {
            UIAuthorizeAccountRequest {
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

    impl From<AccountAuthorizationRequest> for UIDeauthorizeAccountRequest {
        fn from(val: AccountAuthorizationRequest) -> Self {
            UIDeauthorizeAccountRequest {
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

                assert_eq!(signer_address, public_key.to_address().to_hex());
            }

            UserSignature::Simple(SimpleSignature::Secp256k1 { public_key, .. }) => {
                Secp256k1Verifier::new()
                    .verify_personal_message(&personal_message, &signature)
                    .map_err(|_| "Error verifying secp256k1 signature")?;

                assert_eq!(signer_address, public_key.to_address().to_hex());
            }

            _ => Err("Unsupported signature type".to_string())?,
        }

        Ok(())
    }
}
