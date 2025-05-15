use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::trade_api::get_standby_orders;
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

    let open_orders = get_standby_orders(&config, Some(symbol)).await?;
    Ok(open_orders)
}

#[tokio::main]
async fn main() -> Result<()> {
    let environment = Environment::Staging;
    // First, we construct an authentication request.
    let request = LoginRequest {
        account_address: environment.test_keys().unwrap().address.into(),
        audience: auth::audience(environment).into(),
        signed_at_millis: Utc::now().timestamp_millis(),
    };

    // Then, we generate a signature for the request.
    let signature = request.signature(
        SignatureScheme::Ed25519,
        PrivateKey::from_hex(environment.test_keys().unwrap().private_key)?,
    )?;

    // Next, we submit our authentication request to the API for the desired environment.
    let auth_token = request
        .authenticate(&signature, environment)
        .await?
        .access_token;

    // Now, we send the request.
    let open_orders = send_request("ETH-PERP", &auth_token).await?;
    println!("{open_orders:#?}");

    Ok(())
}
