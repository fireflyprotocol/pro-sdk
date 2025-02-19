use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::trade_api::get_open_orders;
use bluefin_api::models::LoginRequest;
use bluefin_api::models::OpenOrderResponse;
use bluefin_pro::prelude::*;
use chrono::Utc;
use hex::FromHex;
use sui_sdk_types::SignatureScheme;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

// Will return an Ok(()) if the request has successfully been submitted to Bluefin
async fn send_request(symbol: &str, auth_token: &str) -> Result<Vec<OpenOrderResponse>> {
    println!("Sending request...");
    // Send request and get back order hash
    let mut config = Configuration::new();
    config.bearer_access_token = Some(auth_token.into());
    config.base_path = trade::testnet::URL.into();

    let open_orders = get_open_orders(&config, Some(symbol)).await?;
    Ok(open_orders)
}

#[tokio::main]
async fn main() -> Result<()> {
    // First, we construct an authentication request.
    let request = LoginRequest {
        account_address: test::account::testnet::ADDRESS.into(),
        audience: auth::testnet::AUDIENCE.into(),
        signed_at_utc_millis: Utc::now().timestamp_millis(),
    };

    // Then, we generate a signature for the request.
    let signature = request.signature(
        SignatureScheme::Ed25519,
        PrivateKey::from_hex(test::account::testnet::PRIVATE_KEY)?,
    )?;

    // Next, we submit our authentication request to the API for the desired environment.
    let auth_token = request
        .authenticate(&signature, Environment::Testnet)
        .await?
        .access_token;

    // Now, we send the request.
    let open_orders = send_request(symbols::perps::ETH, &auth_token).await?;
    println!("{open_orders:#?}");

    Ok(())
}
