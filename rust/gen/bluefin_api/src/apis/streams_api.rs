/*
 * Bluefin API
 *
 * Bluefin API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`web_socket_account_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSocketAccountDataError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`web_socket_market_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSocketMarketDataError {
    UnknownValue(serde_json::Value),
}


/// WebSocket Account Streams URL.
pub async fn web_socket_account_data(configuration: &configuration::Configuration, authorization: &str, upgrade: &str, sec_web_socket_key: &str, sec_web_socket_version: &str) -> Result<(), Error<WebSocketAccountDataError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_authorization = authorization;
    let p_upgrade = upgrade;
    let p_sec_web_socket_key = sec_web_socket_key;
    let p_sec_web_socket_version = sec_web_socket_version;

    let uri_str = format!("{}/ws/account", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Authorization", p_authorization.to_string());
    req_builder = req_builder.header("Upgrade", p_upgrade.to_string());
    req_builder = req_builder.header("Sec-WebSocket-Key", p_sec_web_socket_key.to_string());
    req_builder = req_builder.header("Sec-WebSocket-Version", p_sec_web_socket_version.to_string());
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<WebSocketAccountDataError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// WebSocket Market Streams URL.
pub async fn web_socket_market_data(configuration: &configuration::Configuration, upgrade: &str, sec_web_socket_key: &str, sec_web_socket_version: &str) -> Result<(), Error<WebSocketMarketDataError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_upgrade = upgrade;
    let p_sec_web_socket_key = sec_web_socket_key;
    let p_sec_web_socket_version = sec_web_socket_version;

    let uri_str = format!("{}/ws/market", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Upgrade", p_upgrade.to_string());
    req_builder = req_builder.header("Sec-WebSocket-Key", p_sec_web_socket_key.to_string());
    req_builder = req_builder.header("Sec-WebSocket-Version", p_sec_web_socket_version.to_string());

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<WebSocketMarketDataError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

