use bluefin_api::models::LoginRequest;
use bluefin_pro::prelude::*;
use chrono::Utc;
use hex::FromHex;
use sui_sdk_types::SignatureScheme;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let environment = Environment::Staging;

    let account_address = environment.test_keys().unwrap().address.to_string();
    let private_key = PrivateKey::from_hex(environment.test_keys().unwrap().private_key)?;

    let request = LoginRequest {
        account_address,
        audience: auth::audience(environment).into(),
        signed_at_millis: Utc::now().timestamp_millis(),
    };

    let signature = request.signature(SignatureScheme::Ed25519, private_key)?;

    // Refresh token valid for 10 seconds and an auth token that is read-only.
    let response = request
        .authenticate_with_options(
            &signature,
            environment,
            AuthenticationOptions {
                refresh_token_valid_for_seconds: Some(10),
                read_only: Some(true),
            },
        )
        .await?;

    println!("response: {response:#?}");

    Ok(())
}
