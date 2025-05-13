use bluefin_api::models::{
    MarketDataStreamName, MarketStreamMessage, MarketStreamMessagePayload,
    MarketSubscriptionMessage, MarketSubscriptionStreams, SubscriptionResponseMessage,
    SubscriptionType,
};
use bluefin_pro::prelude::*;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

async fn listen_to_candlestick_updates(
    environment: Environment,
    symbol: &str,
    sender: Sender<MarketStreamMessage>,
    max_time_without_messages: Duration,
    shutdown_flag: Arc<AtomicBool>,
) -> Result<()> {
    let request = ws::market::url(environment).into_client_request()?;

    // Establish connection with websocket URL.
    let (websocket_stream, _) = connect_async(request).await?;
    let (mut ws_sender, mut ws_receiver) = websocket_stream.split();

    // Send a subscription message to receive candlestick updates.
    let sub_message = MarketSubscriptionMessage::new(
        SubscriptionType::Subscribe,
        vec![MarketSubscriptionStreams::new(
            symbol.into(),
            vec![MarketDataStreamName::Candlestick1mLast],
        )],
    );

    ws_sender
        .send(Message::Text(serde_json::to_string(&sub_message)?))
        .await?;

    // Spawn a websocket listener task to listen for messages on the subscribed topic.
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
                    }
                    println!("Pong sent");
                }
                Message::Pong(_) => println!("Pong received"),
                Message::Text(text) => {
                    if let Ok(websocket_message) =
                        serde_json::from_str::<MarketStreamMessage>(&text)
                    {
                        if let Err(error) = sender.send(websocket_message).await {
                            eprintln!("Error sending message to channel: {error}");
                        }
                        continue;
                    }

                    // Check if it's a subscription message.
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
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<MarketStreamMessage>(100);
    listen_to_candlestick_updates(
        environment,
        "ETH-PERP",
        sender,
        Duration::from_secs(10),
        Arc::clone(&shutdown_flag),
    )
    .await?;

    while let Some(websocket_message) = receiver.recv().await {
        if let MarketStreamMessage::CandlestickUpdate {
            payload: MarketStreamMessagePayload::CandlestickUpdate(candlestick),
        } = websocket_message
        {
            println!("{candlestick:#?}");
        }
    }

    shutdown_flag.store(true, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}
