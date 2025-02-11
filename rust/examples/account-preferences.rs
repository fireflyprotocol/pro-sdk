use bluefin_api::apis::account_data_api::get_account_preferences;
use bluefin_api::apis::configuration::Configuration;
use bluefin_api::models::AccountPreference;
use bluefin_api::models::LoginRequest;
use bluefin_pro::{self as bfp, prelude::*};
use chrono::Utc;
use hex::FromHex;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/// Sends a request for account details, and returns the deserialized response.
async fn send_request(auth_token: &str) -> Result<AccountPreference> {
    println!("Sending request...");
    Ok(get_account_preferences(&Configuration {
        base_path: bfp::account::testnet::URL.into(),
        bearer_access_token: Some(auth_token.into()),
        ..Configuration::new()
    })
    .await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let login_request = LoginRequest::new(
        bfp::test::account::testnet::ADDRESS.into(),
        Utc::now().timestamp_millis(),
        bfp::auth::testnet::AUDIENCE.into(),
    );

    let signature = login_request.signature(
        bfp::SignatureType::Ed25519,
        bfp::PrivateKey::from_hex(bfp::test::account::testnet::PRIVATE_KEY)?,
    )?;

    let auth_token = login_request
        .authenticate(&signature, bfp::Environment::Testnet)
        .await?
        .access_token;

    let account = send_request(&auth_token).await?;

    println!("{account:#?}");

    Ok(())
}
