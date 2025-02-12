use bluefin_api::models::{LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse};
use bluefin_pro::{self as bfp, prelude::*};
use chrono::Utc;
use rand::rngs::OsRng;
use secp256k1::Secp256k1;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

async fn auth_secp256k1() -> Result<LoginResponse> {
    // First, we load our private and public keys.
    let secp = Secp256k1::new();
    let (private_key, public_key) = secp.generate_keypair(&mut OsRng);
    let public_key = bfp::Secp256k1PublicKey::from(public_key.serialize());

    // Then, we construct an authentication request.
    let request = LoginRequest {
        account_address: public_key.into_sui_address(),
        audience: bfp::auth::testnet::AUDIENCE.into(),
        signed_at_utc_millis: Utc::now().timestamp_millis(),
    };

    // Next, we generate a signature for the request.
    let signature = request.signature(
        bfp::SignatureType::Secp256k1,
        bfp::PrivateKey::from(private_key.secret_bytes()),
    )?;

    // Now, we submit our authentication request to the API for the desired environment.
    let response = request
        .authenticate(&signature, bfp::Environment::Testnet)
        .await?;

    // Finally, we inspect our response to ensure that we've been properly authenticated.
    println!("secp256k1 Authentication response: {response:#?}");

    Ok(response)
}

async fn auth_ed25519() -> Result<LoginResponse> {
    // First, we load our private and public keys.
    let private_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
    let public_key = bfp::Ed25519PublicKey::from(private_key.verifying_key().to_bytes());

    // Then, we construct an authentication request.
    let request = LoginRequest {
        account_address: public_key.into_sui_address(),
        audience: bfp::auth::testnet::AUDIENCE.into(),
        signed_at_utc_millis: Utc::now().timestamp_millis(),
    };

    // Next, we generate a signature for the request.
    let signature = request.signature(
        bfp::SignatureType::Ed25519,
        bfp::PrivateKey::from(private_key.to_bytes()),
    )?;

    // Now, we submit our authentication request to the API for the desired environment.
    let response = request
        .authenticate(&signature, bfp::Environment::Testnet)
        .await?;

    // Finally, we inspect our response to ensure that we've been properly authenticated.
    println!("ed25519 Authentication response: {response:#?}");

    Ok(response)
}

async fn refresh_token(refresh_token_request: RefreshTokenRequest) -> Result<RefreshTokenResponse> {
    refresh_token_request
        .refresh(bfp::Environment::Testnet)
        .await
        .map_err(|error| error.into())
}

#[tokio::main]
async fn main() -> Result<()> {
    auth_secp256k1().await?;

    let ed25519_login_response = auth_ed25519().await?;

    let refresh_token_request = RefreshTokenRequest {
        refresh_token: ed25519_login_response.refresh_token,
    };

    let refresh_token = refresh_token(refresh_token_request).await?;
    println!("Refresh Token: {refresh_token:#?}");

    Ok(())
}
