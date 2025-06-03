mod shutdown;
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

/// This function will open a websocket connection to Bluefin's market streams and will subscribe to
/// Mark Price Updates.
///
/// The websocket server will send Pings to the client every 10 seconds, so the client should listen
/// to Ping frames and response with Pong frames.
///
/// After subscribing to a topic(s), the first message back will always be a
/// [`SubscriptionResponseMessage`]. The `success` field determines if the subscription was
/// successful or not. The `message` field will contain all the subscribed to streams
///
/// The websocket market streams will always send data using [`MarketStreamMessage`], containing
/// `event` field. The client can use the `event` field to deserialize the `payload` field. Each
///  `event` contains a unique `payload` structure.
///
/// Sample Mark Price output:
///
/// ```json
/// {
///     "event": "MarkPrice",
///     "payload": {
///         "updated_at_millis": 1734048844,
///         "symbol": "ETH-PERP",
///         "price_e9": "3879229230470"
///     }
/// }
/// ```
async fn listen_to_mark_price_updates(
    environment: Environment,
    symbol: &str,
    sender: Sender<MarketStreamMessage>,
    max_time_without_message: Duration,
    shutdown_flag: Arc<AtomicBool>,
) -> Result<()> {
    let request = ws::market::url(environment).into_client_request()?;

    // Establish connection with websocket URL.
    let (websocket_stream, _) = connect_async(request).await?;
    let (mut ws_sender, mut ws_receiver) = websocket_stream.split();

    // Send a subscription message to receive Mark price updates.
    let sub_message = serde_json::to_string(&MarketSubscriptionMessage::new(
        SubscriptionType::Subscribe,
        vec![MarketSubscriptionStreams::new(
            symbol.into(),
            vec![MarketDataStreamName::MarkPrice],
        )],
    ))?;
    ws_sender.send(Message::Text(sub_message)).await?;

    // Spawn a websocket listener task to listen for messages on the subscribed topic.
    tokio::spawn(async move {
        while !shutdown_flag.load(std::sync::atomic::Ordering::Relaxed) {
            let Ok(message) = timeout(max_time_without_message, ws_receiver.next()).await else {
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
                    // Check if it's the Mark price update.
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
    listen_to_mark_price_updates(
        environment,
        "ETH-PERP",
        sender,
        Duration::from_secs(5),
        Arc::clone(&shutdown_flag),
    )
    .await?;

    shutdown::execute(&shutdown_flag, shutdown::DEFAULT_TIMEOUT_SEC);

    while let Some(websocket_message) = receiver.recv().await {
        if let MarketStreamMessage::MarkPriceUpdate {
            payload: MarketStreamMessagePayload::MarkPriceUpdate(mark_price),
        } = websocket_message
        {
            println!("{mark_price:#?}");
        }
    }

    shutdown_flag.store(true, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}
