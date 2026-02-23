use bluefin_api::apis::configuration::Configuration;
use bluefin_api::apis::exchange_api::get_leaderboard;
use bluefin_pro::prelude::*;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let environment = Environment::Staging;
    let response = get_leaderboard(
        &Configuration {
            base_path: exchange::url(environment).into(),
            ..Configuration::default()
        },
        None, // interval
        None, // sort by
        None, // sort order
        None, // limit
        None, // page
    )
    .await?;

    println!("{response:#?}");

    Ok(())
}
