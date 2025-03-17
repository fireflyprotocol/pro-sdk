use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::trade_api::post_create_order;
use bluefin_api::models::{
    CreateOrderRequest, CreateOrderRequestSignedFields, LoginRequest, MarketDataStreamName,
    MarketStreamMessage, MarketStreamMessagePayload, MarketSubscriptionMessage,
    MarketSubscriptionStreams, OrderSide, OrderTimeInForce, OrderType, SelfTradePreventionType,
    SubscriptionResponseMessage, SubscriptionType,
};
use bluefin_pro::prelude::*;
use chrono::{TimeDelta, Utc};
use futures_util::{SinkExt, StreamExt};
use hex::FromHex;
use rand::random;
use std::ops::Add;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;
use sui_sdk_types::SignatureScheme;
use tokio::sync::mpsc::Sender;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn listen_to_partial_depth_updates(
    environment: Environment,
    symbol: &str,
    sender: Sender<MarketStreamMessage>,
    max_time_without_message: Duration,
    shutdown_flag: Arc<AtomicBool>,
) -> Result<()> {
    let request = ws::market::url(environment).into_client_request()?;

    // Establish connection with WebSocket URL.
    let (websocket_stream, _) = connect_async(request).await?;
    let (mut ws_sender, mut ws_receiver) = websocket_stream.split();

    let subscription_message = serde_json::to_string(&MarketSubscriptionMessage::new(
        SubscriptionType::Subscribe,
        vec![MarketSubscriptionStreams::new(
            symbol.into(),
            vec![MarketDataStreamName::PartialDepth5],
        )],
    ))?;

    println!("Sending subscription message: {subscription_message}");

    // Send a subscription message to receive account order updates.
    ws_sender.send(Message::Text(subscription_message)).await?;

    // Spawn a WebSocket listener task to listen for messages on the subscribed topic.
    tokio::spawn(async move {
        while !shutdown_flag.load(std::sync::atomic::Ordering::Relaxed) {
            let Ok(message) = timeout(max_time_without_message, ws_receiver.next()).await else {
                println!("Websocket receiver task timed out due to inactivity");
                return;
            };
            let Some(Ok(message)) = message else {
                println!("Websocket receiver task terminated");
                return;
            };
            match message {
                Message::Ping(_) => {
                    println!("Ping received");
                    // Send back Pong.
                    if let Err(error) = ws_sender.send(Message::Pong(Vec::new())).await {
                        eprintln!("Error sending Pong: {error}");
                        return;
                    }
                    println!("Pong sent");
                }
                Message::Pong(_) => {
                    println!("Pong received");
                }
                Message::Text(text) => {
                    // Check whether it's a subscription message, or a stream message.
                    if let Ok(websocket_message) =
                        serde_json::from_str::<MarketStreamMessage>(&text)
                    {
                        if let Err(error) = sender.send(websocket_message).await {
                            eprintln!("Error sending message to channel: {error}");
                        }
                        continue;
                    }

                    if let Ok(subscription_message) =
                        serde_json::from_str::<SubscriptionResponseMessage>(&text)
                    {
                        println!(
                            "Subscription response message received: {}",
                            serde_json::to_string_pretty(&subscription_message).unwrap()
                        );
                    }
                }
                Message::Close(_) => {
                    println!("Close received");
                    return;
                }
                _ => {
                    eprintln!("Unknown message received");
                    return;
                }
            }
        }
    });

    Ok(())
}

async fn create_order(signed_request: CreateOrderRequest, auth_token: &str) -> Result<String> {
    println!("Sending request...");
    // Send request and get back order hash
    let mut config = Configuration::new();
    config.bearer_access_token = Some(auth_token.into());
    config.base_path = trade::testnet::URL.into();

    let response = post_create_order(&config, signed_request).await?;

    Ok(response.order_hash)
}

#[tokio::main]
async fn main() -> Result<()> {
    // We construct an authentication request to obtain a token.
    let request = LoginRequest {
        account_address: test::account::testnet::ADDRESS.into(),
        audience: auth::testnet::AUDIENCE.into(),
        signed_at_millis: Utc::now().timestamp_millis(),
    };

    // Next, we generate a signature for the request.
    let signature = request.signature(
        SignatureScheme::Ed25519,
        PrivateKey::from_hex(test::account::testnet::PRIVATE_KEY)?,
    )?;

    // Then, we submit our authentication request to the API for the desired environment.
    let auth_token = request
        .authenticate(&signature, Environment::Testnet)
        .await?
        .access_token;

    // We connect to the market stream WebSocket to listen for partial depth.
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<MarketStreamMessage>(100);
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    listen_to_partial_depth_updates(
        Environment::Testnet,
        symbols::perps::ETH,
        sender,
        Duration::from_secs(15),
        Arc::clone(&shutdown_flag),
    )
    .await?;

    let handle = tokio::spawn(async move {
        while let Some(websocket_message) = receiver.recv().await {
            if let MarketStreamMessage::OrderbookPartialDepthUpdate {
                payload: MarketStreamMessagePayload::OrderbookPartialDepthUpdate(msg),
            } = websocket_message
            {
                println!("{msg:#?}");
            }
        }
    });

    // We get the exchange info to fetch the IDS_ID
    let contracts_info = exchange::info::contracts_config(Environment::Testnet).await?;

    // Create an order to update the orderbook
    let request = CreateOrderRequest {
        signed_fields: CreateOrderRequestSignedFields {
            symbol: symbols::perps::ETH.into(),
            account_address: test::account::testnet::ADDRESS.into(),
            price_e9: (10_000.e9()).to_string(),
            quantity_e9: (1.e9()).to_string(),
            side: OrderSide::Short,
            leverage_e9: (10.e9()).to_string(),
            is_isolated: false,
            salt: random::<u64>().to_string(),
            ids_id: contracts_info.ids_id,
            expires_at_millis: Utc::now().add(TimeDelta::minutes(5)).timestamp_millis(),
            signed_at_millis: Utc::now().timestamp_millis(),
        },
        signature: String::new(),
        order_hash: String::new(),
        client_order_id: None,
        r#type: OrderType::Limit,
        reduce_only: false,
        post_only: true,
        time_in_force: OrderTimeInForce::Gtt,
        trigger_price_e9: None,
        self_trade_prevention_type: Some(SelfTradePreventionType::Maker),
    };

    // Then, we sign our order.
    let request = request.sign(
        PrivateKey::from_hex(test::account::testnet::PRIVATE_KEY)?,
        SignatureScheme::Ed25519,
    )?;
    let order_hash = create_order(request, &auth_token).await?;
    println!("Created Order: {order_hash}");

    shutdown_flag.store(true, std::sync::atomic::Ordering::Relaxed);
    handle.await.expect("Could not join handle");

    Ok(())
}
