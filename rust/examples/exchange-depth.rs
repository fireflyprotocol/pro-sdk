use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::exchange_api::get_orderbook_depth;
use bluefin_pro::prelude::*;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let response = get_orderbook_depth(
        &Configuration {
            base_path: exchange::testnet::URL.into(),
            ..Configuration::default()
        },
        symbols::perps::ETH, // symbol
        Some(5),             // limit
    )
    .await?;

    println!("{response:#?}");

    Ok(())
}
