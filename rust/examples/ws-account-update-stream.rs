use bluefin_api::models::LoginRequest;
use bluefin_api::models::{
    AccountDataStream, AccountStreamMessage, AccountStreamMessagePayload,
    AccountSubscriptionMessage, SubscriptionResponseMessage, SubscriptionType,
};
use bluefin_pro::prelude::*;
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use hex::FromHex;
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

/// Submits a login request to the specified environment, and returns the resulting access token.
/// See the accompanying [`authenticate.rs`] example for a more detailed explanation.
///
/// # Errors
///
/// Will return `Err` if the authentication cannot be reached, or rejects our request.
async fn authenticate(environment: Environment) -> Result<String> {
    let audience = auth::audience(environment);
    let request = LoginRequest {
        account_address: environment.test_keys().unwrap().address.into(),
        audience: audience.into(),
        signed_at_millis: Utc::now().timestamp_millis(),
    };
    let signature = request.signature(
        SignatureScheme::Ed25519,
        PrivateKey::from_hex(environment.test_keys().unwrap().private_key)?,
    )?;
    let response = request.authenticate(&signature, environment).await?;
    Ok(response.access_token)
}

async fn listen_to_account_info(
    auth_token: &str,
    environment: Environment,
    sender: Sender<AccountStreamMessage>,
    max_time_without_messages: Duration,
    shutdown_flag: Arc<AtomicBool>,
) -> Result<()> {
    // Then, we establish a connection through the websocket URL for that environment.
    let mut request = ws::account::url(environment).into_client_request()?;
    request.headers_mut().insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {auth_token}"))?,
    );
    let (websocket_stream, _) = connect_async(request).await?;

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
    // First, we log into the desired environment.
    let auth_token = authenticate(environment).await?;

    // Stream websocket messages for 10 seconds
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<AccountStreamMessage>(100);
    listen_to_account_info(
        &auth_token,
        environment,
        sender,
        Duration::from_secs(5),
        Arc::clone(&shutdown_flag),
    )
    .await?;

    // Listen to the mpsc channel for 5 seconds (for the sake of this example) to get the account
    // update. Normally, you would listen to this channel indefinitely to wait for messages.
    while let Some(websocket_message) = receiver.recv().await {
        if let AccountStreamMessage::AccountUpdate {
            payload: AccountStreamMessagePayload::AccountUpdate(account_update),
            ..
        } = websocket_message
        {
            println!("{account_update:#?}",);
        }
    }

    shutdown_flag.store(true, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}
