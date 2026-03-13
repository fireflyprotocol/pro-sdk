use bluefin_api::apis::account_data_api::get_account_value_history_by_account;
use bluefin_api::apis::configuration::Configuration;
use bluefin_api::models::AccountValueHistory;
use bluefin_pro::prelude::*;


type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/// Sends a request for account value history, and returns the deserialized response.
async fn send_request(account_address: &str) -> Result<AccountValueHistory> {
    println!("Sending request...");
    Ok(get_account_value_history_by_account(
        &Configuration {
            base_path: account::staging::URL.into(),
            bearer_access_token: Some(account_address.into()),
            ..Configuration::new()
        },
        account_address,
    )
    .await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let environment = Environment::Staging;
    let test_account_address = environment.test_keys().unwrap().address;

    let response = send_request(test_account_address).await?;

    println!("{response:#?}");

    Ok(())
}

