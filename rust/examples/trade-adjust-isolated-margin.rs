use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::trade_api::put_adjust_isolated_margin;
use bluefin_api::models::LoginRequest;
use bluefin_api::models::{
    AdjustIsolatedMarginRequest, AdjustIsolatedMarginRequestSignedFields, AdjustMarginOperation,
};
use bluefin_pro::prelude::*;
use chrono::Utc;
use hex::FromHex;
use rand::random;
use sui_sdk_types::SignatureScheme;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

async fn send_request(signed_request: AdjustIsolatedMarginRequest, auth_token: &str) -> Result<()> {
    println!("Sending request...");
    // Send request and get back order hash
    let mut config = Configuration::new();
    config.bearer_access_token = Some(auth_token.into());
    config.base_path = trade::devnet::URL.into();

    put_adjust_isolated_margin(&config, signed_request).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let environment = Environment::Staging;
    // We construct an authentication request to obtain a token.
    let request = LoginRequest {
        account_address: environment.test_keys().unwrap().address.into(),
        audience: auth::audience(environment).into(),
        signed_at_millis: Utc::now().timestamp_millis(),
    };

    // Next, we generate a signature for the request.
    let signature = request.signature(
        SignatureScheme::Ed25519,
        PrivateKey::from_hex(environment.test_keys().unwrap().private_key)?,
    )?;

    // Then, we submit our authentication request to the API for the desired environment.
    let auth_token = request
        .authenticate(&signature, environment)
        .await?
        .access_token;

    // We get the exchange info to fetch the IDS_ID
    let contracts_info = exchange::info::contracts_config(environment).await?;

    // Next, we construct an unsigned request.
    let request = AdjustIsolatedMarginRequest {
        signed_fields: AdjustIsolatedMarginRequestSignedFields {
            symbol: "ETH-PERP".to_string(),
            account_address: environment.test_keys().unwrap().address.into(),
            operation: AdjustMarginOperation::Add,
            quantity_e9: (1.e9()).to_string(),
            salt: random::<u64>().to_string(),
            ids_id: contracts_info.ids_id,
            signed_at_millis: Utc::now().timestamp_millis(),
        },
        ..AdjustIsolatedMarginRequest::default()
    };

    // Then, we sign our order.
    let request = request.sign(
        PrivateKey::from_hex(environment.test_keys().unwrap().private_key)?,
        SignatureScheme::Ed25519,
    )?;

    send_request(request, &auth_token).await?;

    // Finally, we check that we've received the expected order hash.
    println!("Margin Adjusted");
    Ok(())
}
