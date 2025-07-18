use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::exchange_api::get_candlestick_data;
use bluefin_api::models::{CandlePriceType, KlineInterval};
use bluefin_pro::prelude::*;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let environment = Environment::Staging;
    let response = get_candlestick_data(
        &Configuration {
            base_path: exchange::url(environment).into(),
            ..Configuration::default()
        },
        "ETH-PERP",                // symbol
        KlineInterval::Variant12h, // interval
        CandlePriceType::Last,     // type
        None,                      // start_time_at_millis
        None,                      // end_time_at_millis
        None,                      // limit
        None,                      // page
    )
    .await?;

    println!("{response:#?}");

    Ok(())
}
