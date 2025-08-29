use bluefin_api::apis::account_data_api::{get_account_details, patch_account_group_id};
use bluefin_api::apis::configuration::Configuration;
use bluefin_api::models::{AccountGroupIdPatch, LoginRequest};
use bluefin_pro::prelude::*;
use chrono::Utc;
use hex::FromHex;
use sui_sdk_types::SignatureScheme;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let environment = Environment::Staging;
    let test_account_address = environment.test_keys().unwrap().address;

    let request = LoginRequest {
        account_address: environment.test_keys().unwrap().address.into(),
        audience: auth::audience(environment).into(),
        signed_at_millis: Utc::now().timestamp_millis(),
    };

    // Next, we generate a signature for the request.
    let signature = request.signature(
        SignatureScheme::Ed25519,
        PrivateKey::from_hex(environment.test_keys().unwrap().private_key)?,
    )?;

    // Then, we submit our authentication request to the API for the desired environment.
    let auth_token = request
        .authenticate(&signature, environment)
        .await?
        .access_token;

    let configuration = Configuration {
        base_path: account::url(environment).to_string(),
        bearer_access_token: Some(auth_token),
        ..Configuration::new()
    };

    patch_account_group_id(
        &configuration,
        AccountGroupIdPatch {
            account_address: test_account_address.to_string(),
            group_id: Some("test".into()),
        },
    )
    .await?;

    // Check account details to verify the change
    get_account_details(&configuration, Some(test_account_address)).await?;

    Ok(())
}
