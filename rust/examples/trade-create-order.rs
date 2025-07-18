use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::trade_api::post_create_order;
use bluefin_api::models::{
    AccountDataStream, AccountOrderUpdate, AccountStreamMessage, AccountStreamMessagePayload,
    AccountSubscriptionMessage, CreateOrderRequest, OrderSide, OrderTimeInForce, OrderType,
    SelfTradePreventionType, SubscriptionResponseMessage, SubscriptionType,
};
use bluefin_api::models::{CreateOrderRequestSignedFields, LoginRequest};
use bluefin_pro::prelude::*;
use chrono::{TimeDelta, Utc};
use futures_util::{SinkExt, StreamExt};
use hex::FromHex;
use rand::random;
use std::ops::Add;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use sui_sdk_types::SignatureScheme;
use tokio::sync::broadcast;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

async fn send_request(
    signed_request: CreateOrderRequest,
    auth_token: &str,
    environment: Environment,
    is_colocated: bool,
) -> Result<String> {
    println!("Sending request...");
    // Send request and get back order hash
    let mut config = Configuration::new();
    config.bearer_access_token = Some(auth_token.into());
    config.base_path = if is_colocated {
        trade::colocated_url(environment).into()
    } else {
        trade::url(environment).into()
    };

    let response = post_create_order(&config, signed_request).await?;

    Ok(response.order_hash)
}

async fn listen_to_account_order_updates(
    auth_token: &str,
    environment: Environment,
    sender: broadcast::Sender<AccountStreamMessage>,
    max_time_without_messages: Duration,
    shutdown_flag: Arc<AtomicBool>,
) -> Result<()> {
    // We establish a connection through the websocket URL for that environment.
    let mut url = ws::account::url(environment).into_client_request()?;
    url.headers_mut().insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {auth_token}"))?,
    );
    let (websocket_stream, _) = connect_async(url).await?;

    // Next, we send a subscription message to receive account updates.
    let (mut ws_sender, mut ws_receiver) = websocket_stream.split();
    ws_sender
        .send(Message::Text(serde_json::to_string(
            &AccountSubscriptionMessage::new(
                SubscriptionType::Subscribe,
                vec![AccountDataStream::AccountOrderUpdate],
            ),
        )?))
        .await?;

    // Now, we spawn a websocket listener task to listen for messages on the subscribed topic.
    tokio::spawn(async move {
        while !shutdown_flag.load(std::sync::atomic::Ordering::Relaxed) {
            let Ok(message) = timeout(max_time_without_messages, ws_receiver.next()).await else {
                println!("Websocket receiver task timed out due to inactivity.");
                return;
            };
            let Some(Ok(message)) = message else {
                println!("Websocket receiver task terminated");
                return;
            };
            match message {
                Message::Ping(_) => {
                    println!("Ping received");
                    // Send Pong.
                    if let Err(error) = ws_sender.send(Message::Pong(Vec::new())).await {
                        eprintln!("Error sending Pong: {error}");
                        return;
                    }
                    println!("Pong sent");
                }
                Message::Pong(_) => println!("Pong received"),
                Message::Text(text) => {
                    // Check if it's the account update.
                    if let Ok(websocket_message) =
                        serde_json::from_str::<AccountStreamMessage>(&text)
                    {
                        if let Err(error) = sender.send(websocket_message) {
                            eprintln!("Error sending message to channel: {error}");
                            return;
                        }
                    }
                    // Check if it's a subscription message.
                    else if let Ok(subscription_message) =
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

    // Finally, we return without error.
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
    let request = CreateOrderRequest {
        signed_fields: CreateOrderRequestSignedFields {
            symbol: "ETH-PERP".to_string(),
            account_address: environment.test_keys().unwrap().address.into(),
            price_e9: (10_000.e9()).to_string(),
            quantity_e9: (1.e9()).to_string(),
            side: OrderSide::Short,
            leverage_e9: (10.e9()).to_string(),
            is_isolated: false,
            salt: random::<u64>().to_string(),
            ids_id: contracts_info.ids_id,
            expires_at_millis: Utc::now().add(TimeDelta::minutes(6)).timestamp_millis(),
            signed_at_millis: Utc::now().timestamp_millis(),
        },
        client_order_id: None,
        r#type: OrderType::Limit,
        reduce_only: false,
        post_only: Some(true),
        time_in_force: Some(OrderTimeInForce::Gtt),
        trigger_price_e9: None,
        self_trade_prevention_type: Some(SelfTradePreventionType::Maker),
        ..Default::default()
    };

    // Then, we sign our order.
    let request = request.sign(
        PrivateKey::from_hex(environment.test_keys().unwrap().private_key)?,
        SignatureScheme::Ed25519,
    )?;

    // Listen to order updates to see when an order has been opened
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let (sender, mut receiver) = broadcast::channel(20);
    listen_to_account_order_updates(
        &auth_token,
        environment,
        sender,
        Duration::from_secs(10),
        Arc::clone(&shutdown_flag),
    )
    .await?;

    let handle = tokio::spawn(async move {
        // Listen to websocket channel to check if the order hash been opened
        while let Ok(websocket_message) = receiver.recv().await {
            match websocket_message {
                AccountStreamMessage::AccountOrderUpdate {
                    payload:
                        AccountStreamMessagePayload::AccountOrderUpdate(
                            AccountOrderUpdate::ActiveOrderUpdate(order_update),
                        ),
                    ..
                } => {
                    println!("Order {} opened", order_update.order_hash);
                    println!("Account Order Update {order_update:#?}");
                    break;
                }
                AccountStreamMessage::AccountOrderUpdate {
                    payload:
                        AccountStreamMessagePayload::AccountOrderUpdate(
                            AccountOrderUpdate::OrderCancellationUpdate(order_cancelled),
                        ),
                    ..
                } => {
                    eprintln!("Order {} cancelled", order_cancelled.order_hash);
                    break;
                }
                unexpected => {
                    eprintln!("Unknown message received {unexpected:#?}");
                }
            }
        }
    });

    println!("Waiting for account order updates...");
    println!("auth token: {auth_token}");
    let received_order_hash =
        send_request(request.clone(), &auth_token, environment, false).await?;

    // Finally, we check that we've received the expected order hash.
    println!("Order Submitted: {received_order_hash}");
    let computed_hash = request.clone().compute_hash().unwrap();
    assert_eq!(computed_hash, received_order_hash);
    println!("Order hash matches");

    println!("Submitting order through colocated url...");
    let received_order_hash = send_request(request.clone(), &auth_token, environment, true).await?;

    println!("Order Submitted through colocated URL: {received_order_hash}");
    handle.await.expect("Could not join handle");
    Ok(())
}
