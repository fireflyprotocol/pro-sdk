use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::exchange_api::get_market_ticker;
use bluefin_pro::prelude::*;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let environment = Environment::Staging;
    let response = get_market_ticker(
        &Configuration {
            base_path: exchange::url(environment).into(),
            ..Configuration::default()
        },
        "ETH-PERP", // symbol
    )
    .await?;

    println!("{response:#?}");

    Ok(())
}
