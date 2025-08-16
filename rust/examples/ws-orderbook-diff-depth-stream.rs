use bluefin_api::models::{
    MarketDataStreamName, MarketStreamMessage, MarketStreamMessagePayload,
    MarketSubscriptionMessage, MarketSubscriptionStreams, SubscriptionResponseMessage,
    SubscriptionType,
};
use bluefin_pro::prelude::*;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn listen_to_diff_depth_updates(
    environment: Environment,
    symbol: &str,
    sender: Sender<MarketStreamMessage>,
    max_time_without_messages: Duration,
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
            vec![MarketDataStreamName::DiffDepth10Ms],
        )],
    ))?;

    println!("Sending subscription message: {subscription_message}");

    // Send a subscription message to receive diff depth updates.
    ws_sender.send(Message::Text(subscription_message)).await?;

    // Spawn a WebSocket listener task to listen for messages on the subscribed topic.
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
                    // Send back Pong.
                    if let Err(error) = ws_sender.send(Message::Pong(Vec::new())).await {
                        eprintln!("Error sending Pong: {error}");
                        return;
                    }
                    println!("Pong sent");
                }
                Message::Pong(_) => println!("Pong received"),
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

/// Connects to the Public Diff Depth Market websocket stream from 2 different connections
/// and prints the first and last update IDs received from each connection.
#[tokio::main]
async fn main() -> Result<()> {
    let environment = Environment::Production;

    // We connect to the market stream WebSocket to listen for diff depth.
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<MarketStreamMessage>(100);
    let (sender2, mut receiver2) = tokio::sync::mpsc::channel::<MarketStreamMessage>(100);
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    listen_to_diff_depth_updates(
        environment,
        "ETH-PERP",
        sender,
        Duration::from_secs(600),
        Arc::clone(&shutdown_flag),
    )
    .await?;

    listen_to_diff_depth_updates(
        environment,
        "ETH-PERP",
        sender2,
        Duration::from_secs(600),
        Arc::clone(&shutdown_flag),
    )
    .await?;

    let handle = tokio::spawn(async move {
        while let Some(websocket_message) = receiver.recv().await {
            if let MarketStreamMessage::OrderbookDiffDepthUpdate {
                payload: MarketStreamMessagePayload::OrderbookDiffDepthUpdate(msg),
            } = websocket_message
            {
                println!(
                    "Task 1: first_update_id: {}, last_update_id: {}",
                    msg.first_update_id, msg.last_update_id
                );
            }
        }
    });

    let handle2 = tokio::spawn(async move {
        while let Some(websocket_message) = receiver2.recv().await {
            if let MarketStreamMessage::OrderbookDiffDepthUpdate {
                payload: MarketStreamMessagePayload::OrderbookDiffDepthUpdate(msg),
            } = websocket_message
            {
                println!(
                    "Task 2: first_update_id: {}, last_update_id: {}",
                    msg.first_update_id, msg.last_update_id
                );
            }
        }
    });

    // Sleep for 10s while the diff depth updates are being received.
    tokio::time::sleep(Duration::from_secs(10)).await;

    shutdown_flag.store(true, std::sync::atomic::Ordering::Relaxed);
    handle.await.expect("Could not join handle");
    handle2.await.expect("Could not join handle2");
    Ok(())
}
