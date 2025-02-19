use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::exchange_api::get_funding_rate_history;
use bluefin_pro::prelude::*;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let response = get_funding_rate_history(
        &Configuration {
            base_path: exchange::testnet::URL.into(),
            ..Configuration::default()
        },
        symbols::perps::ETH, // symbol
        None,                // limit
        None,                // start_time_at_utc_millis
        None,                // end_time_at_utc_millis
        None,                // page
    )
    .await?;

    println!("{response:#?}");

    Ok(())
}
