use std::{
    ops::Add,
    sync::{Arc, atomic::AtomicBool},
    time::Duration,
};

use bluefin_api::{
    apis::{
        account_data_api::get_account_details,
        configuration::Configuration,
        exchange_api::get_exchange_info,
        trade_api::{get_open_orders, post_create_order, post_withdraw},
    },
    models::{
        AccountDataStream, AccountStreamMessage, AccountStreamMessagePayload,
        AccountSubscriptionMessage, CreateOrderRequest, CreateOrderRequestSignedFields,
        LoginRequest, MarketDataStreamName, MarketStreamMessage, MarketStreamMessagePayload,
        MarketSubscriptionMessage, MarketSubscriptionStreams, OrderSide, OrderTimeInForce,
        OrderType, SubscriptionResponseMessage, SubscriptionType, WithdrawRequest,
        WithdrawRequestSignedFields,
    },
};
use bluefin_pro::prelude::*;
use chrono::{TimeDelta, Utc};
use futures_util::{SinkExt, StreamExt};
use hex::FromHex;
use rand::random;
use sui_sdk_types::SignatureScheme;
use tokio::{sync::mpsc::Sender, time::timeout};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, client::IntoClientRequest, http::HeaderValue},
};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

async fn listen_to_account_stream_websocket(
    url: &str,
    auth_token: &str,
    subscriptions: Vec<AccountDataStream>,
    sender: Sender<AccountStreamMessage>,
    shutdown_signal: Arc<AtomicBool>,
    max_time_without_messages: Duration,
) -> Result<()> {
    // Then, we establish a connection through the websocket URL for that environment.
    let mut request = url.into_client_request()?;
    request.headers_mut().insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {auth_token}"))?,
    );
    let (websocket_stream, _) = connect_async(request).await?;

    // Next, we send a subscription message to receive account updates.
    let (mut ws_sender, mut ws_receiver) = websocket_stream.split();
    ws_sender
        .send(Message::Text(serde_json::to_string(
            &AccountSubscriptionMessage::new(SubscriptionType::Subscribe, subscriptions),
        )?))
        .await?;

    // Now, we spawn a websocket listener task to listen for messages on the subscribed topic.
    tokio::spawn(async move {
        while !shutdown_signal.load(std::sync::atomic::Ordering::Relaxed) {
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
                        if let Err(error) = sender.send(websocket_message).await {
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

async fn listen_to_market_stream_websocket(
    url: &str,
    subscriptions: Vec<MarketSubscriptionStreams>,
    sender: Sender<MarketStreamMessage>,
    shutdown_signal: Arc<AtomicBool>,
    max_time_without_messages: Duration,
) -> Result<()> {
    // Then, we establish a connection through the websocket URL for that environment.
    let request = url.into_client_request()?;
    let (websocket_stream, _) = connect_async(request).await?;

    // Next, we send a subscription message to receive account updates.
    let (mut ws_sender, mut ws_receiver) = websocket_stream.split();
    ws_sender
        .send(Message::Text(serde_json::to_string(
            &MarketSubscriptionMessage::new(SubscriptionType::Subscribe, subscriptions),
        )?))
        .await?;

    // Now, we spawn a websocket listener task to listen for messages on the subscribed topic.
    tokio::spawn(async move {
        while !shutdown_signal.load(std::sync::atomic::Ordering::Relaxed) {
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
                        serde_json::from_str::<MarketStreamMessage>(&text)
                    {
                        if let Err(error) = sender.send(websocket_message).await {
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

/// This example will run a full lifecycle of the sdk, as a user would.
/// It will create an orders, and then cancel them.
/// It will listen to price updates and place orders based on the price.
/// It will listen to account updates and market updates using websockets.
/// This example will listen to a preset account on testnet.
#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() -> Result<()> {
    const ACCOUNT: &str = "0x5eed54482b8671c0d9749dad475f8ba284afb63426ea44d0c18ae6d94641ff2d";
    const PRIVATE_KEY: &str = "3c2306384ae8d2b90060848d13720c433e90a6f3f616d43374bd586c548719d0";

    const ACCOUNT_STREAM_TIMEOUT: u64 = 30;
    const MARKET_STREAM_TIMEOUT: u64 = 30;
    const SLEEP_TIME: u64 = 30;

    let environment = Environment::Staging;
    let account_service_url = account::url(environment);
    let exchange_service_url = exchange::url(environment);
    let trade_service_url = trade::url(environment);
    let ws_market_service_url = ws::market::url(environment);
    let ws_account_service_url = ws::account::url(environment);

    let login_request = LoginRequest::new(
        ACCOUNT.into(),
        Utc::now().timestamp_millis(),
        auth::audience(environment).into(),
    );

    let signature =
        login_request.signature(SignatureScheme::Ed25519, PrivateKey::from_hex(PRIVATE_KEY)?)?;

    let auth_token = login_request
        .authenticate(&signature, environment)
        .await?
        .access_token;

    // ====== Websocket Setup ======

    // Open a connection to account websockets for all Account related events.
    let (account_sender, mut account_receiver) = tokio::sync::mpsc::channel(100);
    let shutdown_signal = Arc::new(AtomicBool::new(false));
    let account_stream_subscriptions = vec![
        AccountDataStream::AccountUpdate,
        AccountDataStream::AccountOrderUpdate,
        AccountDataStream::AccountTradeUpdate,
        AccountDataStream::AccountPositionUpdate,
        AccountDataStream::AccountTransactionUpdate,
    ];

    let websocket_signal = Arc::clone(&shutdown_signal);
    listen_to_account_stream_websocket(
        ws_account_service_url,
        &auth_token,
        account_stream_subscriptions,
        account_sender,
        websocket_signal,
        Duration::from_secs(ACCOUNT_STREAM_TIMEOUT),
    )
    .await?;

    tokio::spawn(async move {
        while let Ok(Some(message)) = timeout(Duration::from_secs(5), account_receiver.recv()).await
        {
            match message {
                AccountStreamMessage::AccountUpdate {
                    payload: AccountStreamMessagePayload::AccountUpdate(_),
                    ..
                } => {
                    println!("Account update received: {message:#?}",);
                }
                AccountStreamMessage::AccountOrderUpdate {
                    payload: AccountStreamMessagePayload::AccountOrderUpdate(_),
                    ..
                } => {
                    println!("Account order update received: {message:#?}",);
                }
                AccountStreamMessage::AccountTradeUpdate {
                    payload: AccountStreamMessagePayload::AccountTradeUpdate(_),
                    ..
                } => {
                    println!("Account trade update received: {message:#?}",);
                }
                AccountStreamMessage::AccountPositionUpdate {
                    payload: AccountStreamMessagePayload::AccountPositionUpdate(_),
                    ..
                } => {
                    println!("Account position update received: {message:#?}",);
                }
                AccountStreamMessage::AccountTransactionUpdate {
                    payload: AccountStreamMessagePayload::AccountTransactionUpdate(_),
                    ..
                } => {
                    println!("Account transaction update received: {message:#?}",);
                }
                _ => {}
            }
        }
    });

    // Open a connection to market websockets for all Market related events.
    let (market_sender, mut market_receiver) = tokio::sync::mpsc::channel(100);
    let websocket_signal = Arc::clone(&shutdown_signal);
    let subscriptions = vec![MarketSubscriptionStreams {
        symbol: "ETH-PERP".to_string(),
        streams: vec![MarketDataStreamName::OraclePrice],
    }];
    listen_to_market_stream_websocket(
        ws_market_service_url,
        subscriptions,
        market_sender,
        websocket_signal,
        Duration::from_secs(MARKET_STREAM_TIMEOUT),
    )
    .await?;

    tokio::spawn(async move {
        while let Ok(Some(message)) = timeout(Duration::from_secs(5), market_receiver.recv()).await
        {
            match message {
                MarketStreamMessage::TickerUpdate {
                    payload: MarketStreamMessagePayload::TickerUpdate(payload),
                } => {
                    println!("Ticker update received: {payload:#?}",);
                }
                MarketStreamMessage::RecentTradesUpdates {
                    payload: MarketStreamMessagePayload::RecentTradesUpdates(payload),
                } => {
                    println!("Recent trades update received: {payload:#?}",);
                }
                MarketStreamMessage::OraclePriceUpdate {
                    payload: MarketStreamMessagePayload::OraclePriceUpdate(payload),
                } => {
                    println!("Oracle price update received: {payload:#?}",);
                }
                _ => {}
            }
        }
    });

    // ===== Get Exchange Info =====
    let exchange_info = get_exchange_info(&Configuration {
        base_path: exchange_service_url.into(),
        bearer_access_token: Some(auth_token.clone()),
        ..Configuration::new()
    })
    .await?;
    println!("Exchange info received: {exchange_info:#?}",);

    // ===== Get Account Details =====
    let account_details = get_account_details(
        &Configuration {
            base_path: account_service_url.into(),
            bearer_access_token: Some(auth_token.clone()),
            ..Configuration::new()
        },
        ACCOUNT.into(),
    )
    .await?;
    println!("Account details received: {account_details:#?}",);

    // ===== Get Open Orders on Account =====
    let open_orders = get_open_orders(
        &Configuration {
            base_path: trade_service_url.into(),
            bearer_access_token: Some(auth_token.clone()),
            ..Configuration::new()
        },
        Some("ETH-PERP"),
    )
    .await?;
    // println!("Open orders received: {open_orders:#?}",);

    // ====== Create Order ======
    let contracts_info = exchange::info::contracts_config(environment).await?;

    let leverage_e9 = open_orders
        .first()
        .map_or(1.e9().to_string(), |order| order.leverage_e9.clone());

    // Next, we construct an unsigned request.
    let request = CreateOrderRequest {
        signed_fields: CreateOrderRequestSignedFields {
            symbol: "ETH-PERP".to_string(),
            account_address: ACCOUNT.into(),
            price_e9: (10_000.e9()).to_string(),
            quantity_e9: 1.e9().to_string(),
            side: OrderSide::Short,
            leverage_e9,
            is_isolated: false,
            salt: Utc::now()
                .timestamp_millis()
                .unsigned_abs()
                .add(random::<u64>())
                .to_string(),
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
        self_trade_prevention_type: None,
        ..Default::default()
    };

    // Then, we sign our order.
    let request = request.sign(PrivateKey::from_hex(PRIVATE_KEY)?, SignatureScheme::Ed25519)?;

    let order_hash = post_create_order(
        &Configuration {
            base_path: trade_service_url.into(),
            bearer_access_token: Some(auth_token.clone()),
            ..Configuration::new()
        },
        request,
    )
    .await?
    .order_hash;
    println!("Order created: Order Hash: {order_hash:#?}",);

    // ====== Withdraw ======
    let withdraw_request = WithdrawRequest {
        signed_fields: WithdrawRequestSignedFields {
            amount_e9: 10.e9().to_string(),
            asset_symbol: "USDC".to_string(),
            account_address: ACCOUNT.into(),
            salt: Utc::now()
                .timestamp_millis()
                .unsigned_abs()
                .add(random::<u64>())
                .to_string(),
            eds_id: contracts_info.eds_id,
            signed_at_millis: Utc::now().timestamp_millis(),
        },
        ..Default::default()
    };

    let withdraw_request =
        withdraw_request.sign(PrivateKey::from_hex(PRIVATE_KEY)?, SignatureScheme::Ed25519)?;

    post_withdraw(
        &Configuration {
            base_path: trade_service_url.into(),
            bearer_access_token: Some(auth_token.clone()),
            ..Configuration::new()
        },
        withdraw_request,
    )
    .await?;
    println!("Withdrawal initiated");

    // Wait for 10 seconds before shutting down.
    tokio::time::sleep(Duration::from_secs(SLEEP_TIME)).await;

    // Shutdown the websocket connections.
    shutdown_signal.store(true, std::sync::atomic::Ordering::Relaxed);

    Ok(())
}
