use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::exchange_api::get_exchange_info;
use bluefin_pro as bfp;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let response = get_exchange_info(&Configuration {
        base_path: bfp::exchange::testnet::URL.into(),
        ..Configuration::default()
    })
    .await?;

    println!("{response:#?}");

    Ok(())
}
