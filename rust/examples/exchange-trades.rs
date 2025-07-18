use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::exchange_api::get_recent_trades;
use bluefin_pro::prelude::*;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let environment = Environment::Staging;
    let response = get_recent_trades(
        &Configuration {
            base_path: exchange::url(environment).into(),
            ..Configuration::default()
        },
        "ETH-PERP", // symbol
        None,       // trade_type
        Some(5),    // limit
        None,       // start_time_at_millis
        None,       // end_time_at_millis
        None,       // page
    )
    .await?;

    println!("{response:#?}");

    Ok(())
}
