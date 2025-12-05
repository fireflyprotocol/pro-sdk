use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::trade_api::post_create_order;
use bluefin_api::models::{
    AccountDataStream, AccountOrderUpdate, AccountStreamMessage, AccountStreamMessagePayload,
    AccountSubscriptionMessage, CreateOrderRequest, OrderSide, OrderTimeInForce, OrderType,
    SelfTradePreventionType, SubscriptionType,
};
use bluefin_api::models::{CreateOrderRequestSignedFields, LoginRequest};
use bluefin_pro::{self as bfp, prelude::*};
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
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

type TcpWebSocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

const MARKET_SYMBOL: &str = "AVAX-PERP";
const MIN_ORDER_PRICE_E9: u64 = 100_000_000;
const TICK_SIZE_E9: u64 = 1_000_000;

pub const KEYS: bfp::KeySet = bfp::KeySet {
    address: "0x5dc56495f5ac595e9fec40a35d68aad077d092ff80c55de6704b6850da2ea30f",
    public_key: "fe2b424eaae95439066d0a82548f8ab5949234840e95d55b03603ca107eaeb87",
    private_key: "4aa8e28e77d0c72a19b6358cc8980a5ab6d9cb53059c5679bd028b3de4c37536",
};

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

/// Requests a WebSocket subscription, and logs received messages.
async fn subscribe_to_updates(
    auth_token: &str,
    environment: Environment,
    max_time_without_messages: Duration,
) -> Result<JoinHandle<()>> {
    // Spawn a handler to read from an internal broadcast channel. Our WebSocket
    // listener will automatically broadcast order updates to the channel.
    let (sender, receiver) = broadcast::channel(20);
    let handle = tokio::spawn(handle_channel(receiver));
    let shutdown_flag = Arc::new(AtomicBool::new(false));

    // Establish a connection through the WebSocket URL for that environment.
    let mut connection = ws::account::url(environment).into_client_request()?;
    let header = HeaderValue::from_str(&format!("Bearer {auth_token}"))?;
    connection.headers_mut().insert("Authorization", header);
    let (socket, _) = connect_async(connection).await?;

    // Send a subscription message to receive account updates.
    let (mut ws_sender, ws_receiver) = socket.split();
    let sub = vec![AccountDataStream::AccountOrderUpdate];
    let sub = AccountSubscriptionMessage::new(SubscriptionType::Subscribe, sub);
    let sub = serde_json::to_string(&sub)?;
    ws_sender.send(Message::Text(sub)).await?;

    // Spawn a WebSocket listener for messages on the subscribed topic. It needs
    // both the WebSocket sender and receiver to support ping/pong (keep-alive).
    tokio::spawn(handle_websocket(
        shutdown_flag,
        ws_sender,
        ws_receiver,
        sender,
        max_time_without_messages,
    ));

    Ok(handle)
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
                println!("{order:?}");
            }
            AccountOrderUpdate::OrderCancellationUpdate(cancel) => {
                println!("{cancel:?}");
            }
        }
    }
}

fn new_create_order_command(
    account_address: String,
    ids_id: String,
    price_e9: u64,
) -> CreateOrderRequest {
    CreateOrderRequest {
        signed_fields: CreateOrderRequestSignedFields {
            symbol: MARKET_SYMBOL.into(),
            account_address,
            price_e9: price_e9.to_string(),
            quantity_e9: (1.e9()).to_string(),
            side: OrderSide::Short,
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

async fn log_in(environment: Environment, address: String, pkey: PrivateKey) -> Result<String> {
    let request = LoginRequest {
        account_address: address,
        audience: auth::audience(environment).into(),
        signed_at_millis: Utc::now().timestamp_millis(),
    };

    let signature = request.signature(SignatureScheme::Ed25519, pkey)?;

    Ok(request
        .authenticate(&signature, environment)
        .await?
        .access_token)
}

#[tokio::main]
async fn main() -> Result<()> {
    let environment = Environment::Staging;

    let Some(keys) = environment.test_keys() else {
        panic!("{environment:?}: test keys not found");
    };

    let pkey = PrivateKey::from_hex(keys.private_key).expect("private key");

    // Subscribe to WebSocket, and log updates asynchronously.
    let auth_token = log_in(environment, keys.address.into(), pkey).await?;
    let handle = subscribe_to_updates(&auth_token, environment, Duration::from_secs(4)).await?;

    // The IDS ID of the target environment is required in all commands.
    let ids_id = exchange::info::contracts_config(environment).await?.ids_id;

    // Place a spate of orders at low prices. We need enough orders (>98) to
    // cause a taker to be canceled with reason `TOO_MANY_MATCHES`.
    //
    // The prices in this loop increase linearly: `y = mx + b`, where `y` is the
    // price, `m` is the tick size, and `b` is the minimum order price.
    for price_e9 in (1..=100).map(|m| m * TICK_SIZE_E9 + MIN_ORDER_PRICE_E9) {
        let command = new_create_order_command(keys.address.into(), ids_id.clone(), price_e9);
        let command = command.sign(pkey, SignatureScheme::Ed25519)?;
        let computed_order_hash = command.clone().compute_hash().unwrap();
        let received_order_hash = send_request(command, &auth_token, environment, false).await?;
        assert_eq!(received_order_hash, computed_order_hash);
    }

    handle.await?;
    Ok(())
}
