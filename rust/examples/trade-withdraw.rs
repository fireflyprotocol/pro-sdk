use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::exchange_api::get_exchange_info;
use bluefin_api::apis::trade_api::post_withdraw;
use bluefin_api::models::{
    AccountDataStream, AccountEventReason, AccountStreamMessage, AccountStreamMessagePayload,
    AccountSubscriptionMessage, LoginRequest, SubscriptionResponseMessage, SubscriptionType,
    WithdrawRequest, WithdrawRequestSignedFields,
};
use bluefin_pro::prelude::*;
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use hex::FromHex;
use rand::random;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;
use sui_sdk_types::SignatureScheme;
use tokio::sync::mpsc::Sender;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::Message;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

// Will return an Ok(()) if the request has successfully been submitted to Bluefin
async fn send_request(
    request: WithdrawRequest,
    auth_token: &str,
    environment: Environment,
) -> Result<()> {
    println!("Sending request...");
    // Send request and get back order hash
    let mut config = Configuration::new();
    config.bearer_access_token = Some(auth_token.into());
    config.base_path = trade::url(environment).into();

    post_withdraw(&config, request).await?;

    Ok(())
}

async fn listen_to_account_info(
    auth_token: &str,
    environment: Environment,
    sender: Sender<AccountStreamMessage>,
    max_time_without_messages: Duration,
    shutdown_flag: Arc<AtomicBool>,
) -> Result<()> {
    // Then, we establish a connection through the websocket URL for that environment.
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
                vec![AccountDataStream::AccountUpdate],
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
                Message::Pong(_) => {
                    println!("Pong received");
                }
                Message::Text(text) => {
                    // Check if it's the account update.
                    if let Ok(websocket_message) =
                        serde_json::from_str::<AccountStreamMessage>(&text)
                    {
                        if let Err(error) = sender.send(websocket_message).await {
                            eprintln!("Error sending message to channel: {error}");
                            return;
                        }
                        continue;

                        // Check if it's a subscription message.
                    } else if let Ok(subscription_message) =
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
    // Then, we construct an authentication request to obtain a token.
    let request = LoginRequest {
        account_address: test::account::address(environment).into(),
        audience: auth::audience(environment).into(),
        signed_at_millis: Utc::now().timestamp_millis(),
    };

    // Next, we generate a signature for the request.
    let signature = request.signature(
        SignatureScheme::Ed25519,
        PrivateKey::from_hex(test::account::private_key(environment))?,
    )?;

    // Then, we submit our authentication request to the API for the desired environment.
    let auth_token = request
        .authenticate(&signature, environment)
        .await?
        .access_token;

    // Stream Account Updates to check if the withdraw went through
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<AccountStreamMessage>(100);
    listen_to_account_info(
        &auth_token,
        environment,
        sender,
        Duration::from_secs(10),
        Arc::clone(&shutdown_flag),
    )
    .await?;

    let handle = tokio::spawn(async move {
        while let Some(websocket_message) = receiver.recv().await {
            if let AccountStreamMessage::AccountUpdate {
                payload: AccountStreamMessagePayload::AccountUpdate(account_update),
                reason,
            } = websocket_message
            {
                if reason == AccountEventReason::Withdraw {
                    println!("{}", serde_json::to_string_pretty(&account_update).unwrap());
                }
            }
        }
    });

    let asset = get_exchange_info(&Configuration {
        base_path: exchange::url(environment).into(),
        ..Configuration::default()
    })
    .await?
    .assets
    .first()
    .ok_or("Missing Asset")?
    .to_owned();

    // We get the exchange info to fetch the EDS_ID
    let contracts_info = exchange::info::contracts_config(environment).await?;

    // Then, we construct a request.
    let request = WithdrawRequest {
        signed_fields: WithdrawRequestSignedFields {
            asset_symbol: asset.symbol.clone(),
            account_address: test::account::address(environment).into(),
            amount_e9: (10.e9()).to_string(),
            salt: random::<u64>().to_string(),
            eds_id: contracts_info.eds_id,
            signed_at_millis: Utc::now().timestamp_millis(),
        },
        ..Default::default()
    };

    let request = request.sign(
        PrivateKey::from_hex(test::account::private_key(environment))?,
        SignatureScheme::Ed25519,
    )?;

    send_request(request, &auth_token, environment).await?;

    shutdown_flag.store(true, std::sync::atomic::Ordering::SeqCst);
    handle.await.unwrap();

    Ok(())
}
