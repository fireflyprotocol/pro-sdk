use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::trade_api::{cancel_orders, post_create_order};
use bluefin_api::models::{
    AccountDataStream, AccountOrderUpdate, AccountStreamMessage, AccountStreamMessagePayload,
    AccountSubscriptionMessage, CancelOrdersRequest, CreateOrderRequest,
    CreateOrderRequestSignedFields, LoginRequest, OrderSide, OrderTimeInForce, OrderType,
    SelfTradePreventionType, SubscriptionResponseMessage, SubscriptionType,
};
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

/// Submits the specified request to Bluefin.
///
/// # Errors
///
/// Will return `Err` if the request fails, or cannot be submitted.
async fn send_request(
    request: CancelOrdersRequest,
    auth_token: &str,
    environment: Environment,
) -> Result<()> {
    println!("Sending request...");
    // Send request and get back order hash
    let mut config = Configuration::new();
    config.bearer_access_token = Some(auth_token.into());
    config.base_path = trade::url(environment).into();
    cancel_orders(&config, request).await?;

    Ok(())
}

async fn send_create_order_request(
    request: CreateOrderRequest,
    auth_token: &str,
    environment: Environment,
) -> Result<String> {
    println!("Sending request...");
    // Send request and get back order hash
    let mut config = Configuration::new();
    config.bearer_access_token = Some(auth_token.into());
    config.base_path = trade::url(environment).into();

    let order_hash = post_create_order(&config, request).await?.order_hash;

    Ok(order_hash)
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
    // Then, we construct an authentication request.
    let request = LoginRequest {
        account_address: environment.test_keys().unwrap().address.into(),
        audience: auth::audience(environment).into(),
        signed_at_millis: Utc::now().timestamp_millis(),
    };

    // Next, we generate a signature for our request.
    let signature = request.signature(
        SignatureScheme::Ed25519,
        PrivateKey::from_hex(environment.test_keys().unwrap().private_key)?,
    )?;

    // Then, we submit our authentication request to the API for the desired environment.
    let auth_token = request
        .authenticate(&signature, environment)
        .await?
        .access_token;

    // Listen to order updates to see when an order has been opened
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let (sender, mut receiver) = broadcast::channel(20);
    let mut cancellation_receiver = sender.subscribe();
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
            if let AccountStreamMessage::AccountOrderUpdate {
                payload:
                    AccountStreamMessagePayload::AccountOrderUpdate(
                        AccountOrderUpdate::ActiveOrderUpdate(order_update),
                    ),
                ..
            } = websocket_message
            {
                println!("Order {} opened", order_update.order_hash);
                break;
            }
        }
    });

    // We get the exchange info to fetch the IDS_ID
    let contracts_info = exchange::info::contracts_config(environment).await?;

    // Let's open an order on the book
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
        signature: String::new(),
        client_order_id: None,
        r#type: OrderType::Limit,
        reduce_only: false,
        post_only: Some(true),
        time_in_force: Some(OrderTimeInForce::Gtt),
        trigger_price_e9: None,
        self_trade_prevention_type: Some(SelfTradePreventionType::Maker),
    };

    // Then, we sign our order.
    let request = request.sign(
        PrivateKey::from_hex(environment.test_keys().unwrap().private_key)?,
        SignatureScheme::Ed25519,
    )?;

    let order_hash = send_create_order_request(request, &auth_token, environment).await?;

    handle.await.expect("Could not join handle");

    // Next, we construct our cancellation request.
    let request = CancelOrdersRequest {
        symbol: "ETH-PERP".to_string(),
        order_hashes: Some(vec![order_hash.clone()]),
    };

    // Now, we submit our cancellation request to Blufin.
    send_request(request, &auth_token, environment).await?;

    // Finally, we print a confirmation message.
    println!("Orders Cancellation submitted successfully.");

    // Normally, you would listen to this channel indefinitely to wait for messages.
    while let Ok(websocket_message) = cancellation_receiver.recv().await {
        if let AccountStreamMessage::AccountOrderUpdate {
            payload:
                AccountStreamMessagePayload::AccountOrderUpdate(
                    AccountOrderUpdate::OrderCancellationUpdate(order_cancellation),
                ),
            ..
        } = websocket_message
            && order_cancellation.order_hash == order_hash
        {
            println!("Order {order_hash} cancelled");
            return Ok(());
        }
    }

    shutdown_flag.store(true, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}
