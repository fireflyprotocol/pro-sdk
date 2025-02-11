use bluefin_api::apis::account_data_api::get_account_trades;
use bluefin_api::apis::configuration::Configuration;
use bluefin_api::models::{LoginRequest, Trade};
use bluefin_pro::{self as bfp, prelude::*};
use chrono::Utc;
use hex::FromHex;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/// Sends a request for account trades, and returns the deserialized response.
async fn send_request(auth_token: &str, symbol: &str) -> Result<Vec<Trade>> {
    println!("Sending request...");
    Ok(get_account_trades(
        &Configuration {
            base_path: bfp::account::testnet::URL.into(),
            bearer_access_token: Some(auth_token.into()),
            ..Configuration::new()
        },
        symbol,
        None,
        None,
        None,
        None,
        None,
    )
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

    let trades = send_request(&auth_token, bfp::symbols::perps::ETH).await?;

    println!("{trades:#?}");

    Ok(())
}
