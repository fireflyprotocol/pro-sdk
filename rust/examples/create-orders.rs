use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::trade_api::post_create_order;
use bluefin_api::models::{
    AccountDataStream, AccountOrderUpdate, AccountStreamMessage, AccountStreamMessagePayload,
    AccountSubscriptionMessage, CreateOrderRequest, OrderSide, OrderTimeInForce, OrderType,
    SelfTradePreventionType, SubscriptionType,
};
use bluefin_api::models::{CreateOrderRequestSignedFields, LoginRequest};
use bluefin_pro::prelude::*;
use chrono::{TimeDelta, Utc};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use hex::FromHex;
use rand::random;
use std::ops::Add;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use sui_sdk_types::SignatureScheme;
use tokio::net::TcpStream;
use tokio::sync::broadcast::{self, Receiver};
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

async fn send_request(
    signed_request: CreateOrderRequest,
    auth_token: &str,
    environment: Environment,
    is_colocated: bool,
) -> Result<String> {
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

type TcpWebSocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Implements our responses to incoming WebSocket messages.
async fn handle_websocket(
    shutdown_flag: Arc<AtomicBool>,
    mut ws_sender: SplitSink<TcpWebSocketStream, Message>,
    mut ws_receiver: SplitStream<TcpWebSocketStream>,
    sender: broadcast::Sender<AccountStreamMessage>,
    max_time_without_messages: Duration,
) {
    while !shutdown_flag.load(std::sync::atomic::Ordering::Relaxed) {
        let Ok(message) = timeout(max_time_without_messages, ws_receiver.next()).await else {
            eprintln!("Websocket receiver task timed out due to inactivity.");
            return;
        };
        let Some(Ok(message)) = message else {
            eprintln!("Websocket receiver task terminated");
            return;
        };
        match message {
            Message::Ping(_) => {
                if let Err(error) = ws_sender.send(Message::Pong(Vec::new())).await {
                    eprintln!("Error sending Pong: {error}");
                    return;
                }
            }
            Message::Text(text) => {
                if let Ok(websocket_message) = serde_json::from_str::<AccountStreamMessage>(&text)
                    && let Err(error) = sender.send(websocket_message)
                {
                    eprintln!("Error sending message to channel: {error}");
                    return;
                }
            }
            Message::Close(_) => return,
            _ => {
                eprintln!("Unknown WebSocket message: {message:?}");
                return;
            }
        }
    }
}

/// Requests a WebSocket subscription, and broadcasts any received
/// [`AccountOrderUpdate`] messages to the sender.
async fn subscribe_to_updates(
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
    let (mut ws_sender, ws_receiver) = websocket_stream.split();
    ws_sender
        .send(Message::Text(serde_json::to_string(
            &AccountSubscriptionMessage::new(
                SubscriptionType::Subscribe,
                vec![AccountDataStream::AccountOrderUpdate],
            ),
        )?))
        .await?;

    // Now, we spawn a websocket listener task to listen for messages on the subscribed topic.
    tokio::spawn(handle_websocket(
        shutdown_flag,
        ws_sender,
        ws_receiver,
        sender,
        max_time_without_messages,
    ));

    // Finally, we return without error.
    Ok(())
}

async fn handle_channel(mut receiver: Receiver<AccountStreamMessage>) {
    while let Ok(message) = receiver.recv().await {
        let AccountStreamMessage::AccountOrderUpdate { payload, .. } = &message else {
            eprintln!("Unknown account stream message: {message:#?}");
            continue;
        };
        let AccountStreamMessagePayload::AccountOrderUpdate(update) = payload else {
            eprintln!("Unknown account stream payload: {message:#?}");
            continue;
        };
        match update {
            AccountOrderUpdate::ActiveOrderUpdate(order) => {
                println!("Order updated: {order:#?}");
                break;
            }
            AccountOrderUpdate::OrderCancellationUpdate(cancel) => {
                println!("Order cancelled: {cancel:?}");
                break;
            }
        }
    }
}

fn new_request(environment: Environment, ids_id: String, price_e9: u64) -> CreateOrderRequest {
    CreateOrderRequest {
        signed_fields: CreateOrderRequestSignedFields {
            symbol: "ETH-PERP".to_string(),
            account_address: environment.test_keys().unwrap().address.into(),
            price_e9: price_e9.to_string(),
            quantity_e9: (1.e9()).to_string(),
            side: OrderSide::Long,
            leverage_e9: (10.e9()).to_string(),
            is_isolated: false,
            salt: random::<u64>().to_string(),
            ids_id,
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
    }
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

    // Listen to order updates to see when an order has been opened. In this
    // example, we first spawn a handler to read from an internal broadcast
    // channel, and then subscribe to updates via WebSocket. Our WebSocket
    // handler automatically broadcasts order updates to the channel.
    let (sender, receiver) = broadcast::channel(20);
    let handle = tokio::spawn(handle_channel(receiver));
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    subscribe_to_updates(
        &auth_token,
        environment,
        sender,
        Duration::from_secs(10),
        Arc::clone(&shutdown_flag),
    )
    .await?;

    // Next, we construct an unsigned request using the exchange's IDS ID.
    let ids_id = exchange::info::contracts_config(environment).await?.ids_id;
    let request = new_request(environment, ids_id, 10_000.e9());

    // Then, we sign our order.
    let request = request.sign(
        PrivateKey::from_hex(environment.test_keys().unwrap().private_key)?,
        SignatureScheme::Ed25519,
    )?;

    let computed_order_hash = request.clone().compute_hash().unwrap();
    let received_order_hash = send_request(request, &auth_token, environment, false).await?;

    // Finally, we check that we've received the expected order hash.
    assert_eq!(computed_order_hash, received_order_hash);

    handle.await.expect("Could not join handle");
    Ok(())
}
