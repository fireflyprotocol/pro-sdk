use bluefin_api::apis::account_data_api::get_account_details;
use bluefin_api::apis::configuration::Configuration;
use bluefin_api::models::Account;
use bluefin_api::models::LoginRequest;
use bluefin_pro::prelude::*;
use chrono::Utc;
use hex::FromHex;
use sui_sdk_types::SignatureScheme;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/// Sends a request for account details, and returns the deserialized response.
async fn send_request(auth_token: &str) -> Result<Account> {
    println!("Sending request...");
    Ok(get_account_details(&Configuration {
        base_path: account::testnet::URL.into(),
        bearer_access_token: Some(auth_token.into()),
        ..Configuration::new()
    })
    .await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let login_request = LoginRequest::new(
        test::account::testnet::ADDRESS.into(),
        Utc::now().timestamp_millis(),
        auth::testnet::AUDIENCE.into(),
    );
    let signature = login_request.signature(
        SignatureScheme::Ed25519,
        PrivateKey::from_hex(test::account::testnet::PRIVATE_KEY)?,
    )?;

    let auth_token = login_request
        .authenticate(&signature, Environment::Testnet)
        .await?
        .access_token;

    let account = send_request(&auth_token).await?;

    println!("{account:#?}");

    Ok(())
}
