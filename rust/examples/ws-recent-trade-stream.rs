use bluefin_api::models::{
    MarketDataStreamName, MarketStreamMessage, MarketStreamMessagePayload,
    MarketSubscriptionMessage, MarketSubscriptionStreams, SubscriptionResponseMessage,
    SubscriptionType,
};
use bluefin_pro::prelude::*;
use futures_util::{SinkExt, StreamExt};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn listen_to_recent_trade_updates(
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
            vec![MarketDataStreamName::RecentTrade],
        )],
    ))?;

    println!("Sending subscription message: {subscription_message}");

    // Send a subscription message to receive recent trade updates.
    ws_sender.send(Message::Text(subscription_message)).await?;

    // Spawn a WebSocket listener task to listen for messages on the subscribed topic.
    tokio::spawn(async move {
        while !shutdown_flag.load(std::sync::atomic::Ordering::Relaxed) {
            let Ok(message) = timeout(max_time_without_messages, ws_receiver.next()).await else {
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

#[tokio::main]
async fn main() -> Result<()> {
    let environment = Environment::Staging;
    // Then, we connect to the market stream WebSocket to listen for ticker.
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<MarketStreamMessage>(100);
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    listen_to_recent_trade_updates(
        environment,
        "ETH-PERP",
        sender,
        Duration::from_secs(5),
        Arc::clone(&shutdown_flag),
    )
    .await?;

    while let Some(websocket_message) = receiver.recv().await {
        if let MarketStreamMessage::RecentTradesUpdates {
            payload: MarketStreamMessagePayload::RecentTradesUpdates(message),
        } = websocket_message
        {
            println!("{message:#?}");
        }
    }
    shutdown_flag.store(true, std::sync::atomic::Ordering::Relaxed);

    Ok(())
}
