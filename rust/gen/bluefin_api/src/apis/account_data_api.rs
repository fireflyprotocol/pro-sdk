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


/// struct for typed errors of method [`get_account_details`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountDetailsError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_account_funding_rate_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountFundingRateHistoryError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_account_preferences`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountPreferencesError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_account_trades`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountTradesError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_account_transaction_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountTransactionHistoryError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}


pub async fn get_account_details(configuration: &configuration::Configuration, ) -> Result<models::Account, Error<GetAccountDetailsError>> {

    let uri_str = format!("{}/api/v1/account", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAccountDetailsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_account_funding_rate_history(configuration: &configuration::Configuration, account_address: Option<&str>, limit: Option<u32>, page: Option<u32>) -> Result<models::AccountFundingRateHistory, Error<GetAccountFundingRateHistoryError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_account_address = account_address;
    let p_limit = limit;
    let p_page = page;

    let uri_str = format!("{}/api/v1/account/fundingRateHistory", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_account_address {
        req_builder = req_builder.query(&[("accountAddress", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAccountFundingRateHistoryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_account_preferences(configuration: &configuration::Configuration, ) -> Result<models::AccountPreference, Error<GetAccountPreferencesError>> {

    let uri_str = format!("{}/api/v1/account/preferences", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAccountPreferencesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_account_trades(configuration: &configuration::Configuration, symbol: Option<&str>, start_time_at_millis: Option<u32>, end_time_at_millis: Option<u32>, limit: Option<u32>, trade_type: Option<models::TradeType>, page: Option<u32>) -> Result<Vec<models::Trade>, Error<GetAccountTradesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_start_time_at_millis = start_time_at_millis;
    let p_end_time_at_millis = end_time_at_millis;
    let p_limit = limit;
    let p_trade_type = trade_type;
    let p_page = page;

    let uri_str = format!("{}/api/v1/account/trades", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_start_time_at_millis {
        req_builder = req_builder.query(&[("startTimeAtMillis", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_time_at_millis {
        req_builder = req_builder.query(&[("endTimeAtMillis", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_trade_type {
        req_builder = req_builder.query(&[("tradeType", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAccountTradesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_account_transaction_history(configuration: &configuration::Configuration, types: Option<Vec<models::TransactionType>>, asset_symbol: Option<&str>, start_time_at_millis: Option<u32>, end_time_at_millis: Option<u32>, limit: Option<u32>, page: Option<u32>) -> Result<Vec<models::Transaction>, Error<GetAccountTransactionHistoryError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_types = types;
    let p_asset_symbol = asset_symbol;
    let p_start_time_at_millis = start_time_at_millis;
    let p_end_time_at_millis = end_time_at_millis;
    let p_limit = limit;
    let p_page = page;

    let uri_str = format!("{}/api/v1/account/transactions", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_types {
        req_builder = match "multi" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("types".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("types", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref param_value) = p_asset_symbol {
        req_builder = req_builder.query(&[("assetSymbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_start_time_at_millis {
        req_builder = req_builder.query(&[("startTimeAtMillis", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_time_at_millis {
        req_builder = req_builder.query(&[("endTimeAtMillis", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAccountTransactionHistoryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

