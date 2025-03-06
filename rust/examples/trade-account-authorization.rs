use bluefin_api::{
    apis::{
        configuration::Configuration,
        trade_api::{put_authorize_account, put_deauthorize_account},
    },
    models::{AccountAuthorizationRequest, AccountAuthorizationRequestSignedFields, LoginRequest},
};
use bluefin_pro::prelude::*;
use chrono::Utc;
use hex::FromHex;
use sui_sdk_types::SignatureScheme;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// # Trade Account Authorization Example
///
/// This example demonstrates how to authorize and deauthorize trading accounts
/// using the Bluefin API.
///
/// ## Key Components
///
/// - `AccountAuthorizationRequestExt`: A wrapper around `AccountAuthorizationRequest` that
///   provides signing capabilities. Always use this wrapper instead of using the internal
///   `AccountAuthorizationRequest` directly. The internal request should only be used when
///   constructing the `AccountAuthorizationRequestExt`. To authorize an account, set the
///   `is_authorize` parameter to `true`. To deauthorize an account, set the `is_authorize`
///   parameter to `false`.
///
/// - The example shows both authorization and deauthorization of an account.
///
async fn send_request(
    request: AccountAuthorizationRequestExt,
    auth_token: &str,
    environment: Environment,
) -> Result<()> {
    let mut config = Configuration::new();
    config.bearer_access_token = Some(auth_token.into());
    config.base_path = trade::url(environment).into();

    if request.is_authorize {
        put_authorize_account(&config, request.request).await?;
    } else {
        put_deauthorize_account(&config, request.request).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let contracts_info = exchange::info::contracts_config(Environment::Devnet).await?;

    let login_request = LoginRequest {
        account_address: test::account::devnet::ADDRESS.into(),
        audience: auth::devnet::AUDIENCE.into(),
        signed_at_utc_millis: Utc::now().timestamp_millis(),
    };

    let signature = login_request.signature(
        SignatureScheme::Ed25519,
        PrivateKey::from_hex(test::account::devnet::PRIVATE_KEY)?,
    )?;

    let auth_token = login_request
        .authenticate(&signature, Environment::Devnet)
        .await?
        .access_token;

    let environment = Environment::Devnet;
    let time_now = Utc::now().timestamp_millis();

    // Authorize account
    let request = AccountAuthorizationRequestExt::new(
        AccountAuthorizationRequest {
            signed_fields: AccountAuthorizationRequestSignedFields {
                account_address: test::account::devnet::ADDRESS.into(),
                authorized_account_address: test::account::devnet::ADDRESS.into(),
                salt: rand::random::<u64>().to_string(),
                ids_id: contracts_info.ids_id.clone(),
                signed_at_utc_millis: time_now,
            },
            ..Default::default()
        },
        true,
    );

    let request = request.sign(
        PrivateKey::from_hex(test::account::devnet::PRIVATE_KEY)?,
        SignatureScheme::Ed25519,
    )?;

    send_request(request, &auth_token, environment).await?;
    println!("Authorized account");

    // Deauthorize account
    let request = AccountAuthorizationRequestExt::new(
        AccountAuthorizationRequest {
            signed_fields: AccountAuthorizationRequestSignedFields {
                account_address: test::account::devnet::ADDRESS.into(),
                authorized_account_address: test::account::devnet::ADDRESS.into(),
                salt: rand::random::<u64>().to_string(),
                ids_id: contracts_info.ids_id.clone(),
                signed_at_utc_millis: time_now,
            },
            ..Default::default()
        },
        false,
    );

    let request = request.sign(
        PrivateKey::from_hex(test::account::devnet::PRIVATE_KEY)?,
        SignatureScheme::Ed25519,
    )?;

    send_request(request, &auth_token, environment).await?;
    println!("Deauthorized account");

    Ok(())
}
