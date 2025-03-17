use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::trade_api::put_leverage_update;
use bluefin_api::models::LoginRequest;
use bluefin_api::models::{
    AccountPositionLeverageUpdateRequest, AccountPositionLeverageUpdateRequestSignedFields,
};
use bluefin_pro::prelude::*;
use chrono::Utc;
use hex::FromHex;
use rand::random;
use sui_sdk_types::SignatureScheme;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

// Will return an Ok(()) if the request has successfully been submitted to Bluefin
async fn send_request(
    request: AccountPositionLeverageUpdateRequest,
    auth_token: &str,
) -> Result<()> {
    println!("Sending request...");
    // Send request and get back order hash
    let mut config = Configuration::new();
    config.bearer_access_token = Some(auth_token.into());
    config.base_path = trade::testnet::URL.into();

    put_leverage_update(&config, request).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // First, we construct an authentication request.
    let request = LoginRequest {
        account_address: test::account::testnet::ADDRESS.into(),
        audience: auth::testnet::AUDIENCE.into(),
        signed_at_millis: Utc::now().timestamp_millis(),
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

    // We get the exchange info to fetch the IDS_ID
    let contracts_info = exchange::info::contracts_config(Environment::Testnet).await?;

    // Then, we construct the request.
    let signed_request = {
        let unsigned_request = AccountPositionLeverageUpdateRequest {
            signed_fields: AccountPositionLeverageUpdateRequestSignedFields {
                symbol: symbols::perps::ETH.into(),
                account_address: test::account::testnet::ADDRESS.into(),
                leverage_e9: (10.e9()).to_string(),
                salt: random::<u64>().to_string(),
                ids_id: contracts_info.ids_id,
                signed_at_millis: Utc::now().timestamp_millis(),
            },
            ..Default::default()
        };

        unsigned_request.sign(
            PrivateKey::from_hex(test::account::testnet::PRIVATE_KEY)?,
            SignatureScheme::Ed25519,
        )?
    };

    // Now, we send the request.
    send_request(signed_request, &auth_token).await?;
    println!("Leverage Updated");

    Ok(())
}
