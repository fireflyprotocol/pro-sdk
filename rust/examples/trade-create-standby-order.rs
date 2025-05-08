use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::trade_api::post_create_order;
use bluefin_api::models::{
    CreateOrderRequest, CreateOrderRequestSignedFields, LoginRequest, OrderSide, OrderType,
    SelfTradePreventionType,
};
use bluefin_pro::prelude::*;
use chrono::{TimeDelta, Utc};
use hex::FromHex;
use rand::random;
use std::ops::Add;
use sui_sdk_types::SignatureScheme;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

async fn send_create_order_request(
    request: CreateOrderRequest,
    auth_token: &str,
) -> Result<String> {
    println!("Sending request...");
    // Send request and get back order hash
    let mut config = Configuration::new();
    config.bearer_access_token = Some(auth_token.into());
    config.base_path = trade::testnet::URL.into();

    let order_hash = post_create_order(&config, request).await?.order_hash;

    Ok(order_hash)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Then, we construct an authentication request.
    let request = LoginRequest {
        account_address: test::account::testnet::ADDRESS.into(),
        audience: auth::testnet::AUDIENCE.into(),
        signed_at_millis: Utc::now().timestamp_millis(),
    };

    // Next, we generate a signature for our request.
    let signature = request.signature(
        SignatureScheme::Ed25519,
        PrivateKey::from_hex(test::account::testnet::PRIVATE_KEY)?,
    )?;

    // Then, we submit our authentication request to the API for the desired environment.
    let auth_token = request
        .authenticate(&signature, Environment::Testnet)
        .await?
        .access_token;

    // We get the exchange info to fetch the IDS_ID
    let contracts_info = exchange::info::contracts_config(Environment::Testnet).await?;

    // Let's open an order on the book
    let request = CreateOrderRequest {
        signed_fields: CreateOrderRequestSignedFields {
            symbol: symbols::perps::ETH.into(),
            account_address: test::account::testnet::ADDRESS.into(),
            price_e9: ("0").to_string(),
            quantity_e9: (1.e9()).to_string(),
            side: OrderSide::Short,
            leverage_e9: (10.e9()).to_string(),
            is_isolated: false,
            salt: random::<u64>().to_string(),
            ids_id: contracts_info.ids_id,
            expires_at_millis: Utc::now().add(TimeDelta::minutes(6)).timestamp_millis(),
            signed_at_millis: Utc::now().timestamp_millis(),
        },
        signature: String::new(),
        client_order_id: None,
        r#type: OrderType::StopMarket,
        reduce_only: true,
        post_only: None,
        time_in_force: None,
        trigger_price_e9: Some((10_000.e9()).to_string()),
        self_trade_prevention_type: Some(SelfTradePreventionType::Maker),
    };

    // Then, we sign our order.
    let request = request.sign(
        PrivateKey::from_hex(test::account::testnet::PRIVATE_KEY)?,
        SignatureScheme::Ed25519,
    )?;

    let order_hash = send_create_order_request(request, &auth_token).await?;

    // Finally, we print a confirmation message.
    println!("Order created successfully with hash: {order_hash}");

    Ok(())
}
