use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::exchange_api::get_candlestick_data;
use bluefin_api::models::{CandlePriceType, KlineInterval};
use bluefin_pro as bfp;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let response = get_candlestick_data(
        &Configuration {
            base_path: bfp::exchange::testnet::URL.into(),
            ..Configuration::default()
        },
        bfp::symbols::perps::ETH,  // symbol
        KlineInterval::Variant12h, // interval
        CandlePriceType::Last,     // type
        None,                      // start_time_at_utc_millis
        None,                      // end_time_at_utc_millis
        None,                      // limit
        None,                      // page
    )
    .await?;

    println!("{response:#?}");

    Ok(())
}
