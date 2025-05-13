use bluefin_api::apis::account_data_api::get_account_funding_rate_history;
use bluefin_api::apis::configuration::Configuration;
use bluefin_api::models::{AccountFundingRateHistory, LoginRequest};
use bluefin_pro::prelude::*;
use chrono::Utc;
use hex::FromHex;
use sui_sdk_types::SignatureScheme;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/// Sends a request for account trades, and returns the deserialized response.
async fn send_request(auth_token: &str) -> Result<AccountFundingRateHistory> {
    println!("Sending request...");
    Ok(get_account_funding_rate_history(
        &Configuration {
            base_path: account::testnet::URL.into(),
            bearer_access_token: Some(auth_token.into()),
            ..Configuration::new()
        },
        None,
        None,
        None,
    )
    .await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let environment = Environment::Staging;
    let test_account_address = environment.test_keys().unwrap().address;
    let login_request = LoginRequest::new(
        test_account_address.into(),
        Utc::now().timestamp_millis(),
        auth::audience(environment).into(),
    );

    let signature = login_request.signature(
        SignatureScheme::Ed25519,
        PrivateKey::from_hex(environment.test_keys().unwrap().private_key)?,
    )?;

    let auth_token = login_request
        .authenticate(&signature, environment)
        .await?
        .access_token;

    let response = send_request(&auth_token).await?;

    println!("{response:#?}");

    Ok(())
}
