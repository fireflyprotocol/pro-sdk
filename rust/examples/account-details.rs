use bluefin_api::apis::account_data_api::get_account_details;
use bluefin_api::apis::configuration::Configuration;
use bluefin_api::models::Account;
use bluefin_pro::prelude::*;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/// Sends a request for account details, and returns the deserialized response.
async fn send_request(account_address: &str, environment: Environment) -> Result<Account> {
    println!("Sending request...");
    Ok(get_account_details(
        &Configuration {
            base_path: account::url(environment).to_string(),
            ..Configuration::new()
        },
        Some(account_address),
    )
    .await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let environment = Environment::Staging;
    let test_account_address = test::account::address(environment);
    let account = send_request(test_account_address, environment).await?;

    println!("{account:#?}");

    Ok(())
}
