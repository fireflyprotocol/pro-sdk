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
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`get_candlestick_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCandlestickDataError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_exchange_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExchangeInfoError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_funding_rate_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFundingRateHistoryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_market_ticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMarketTickerError {
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_orderbook_depth`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrderbookDepthError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_recent_trades`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRecentTradesError {
    UnknownValue(serde_json::Value),
}


pub async fn get_candlestick_data(configuration: &configuration::Configuration, symbol: &str, interval: models::KlineInterval, r#type: models::CandlePriceType, start_time_at_millis: Option<u64>, end_time_at_millis: Option<u64>, limit: Option<u32>, page: Option<u32>) -> Result<Vec<Vec<String>>, Error<GetCandlestickDataError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_interval = interval;
    let p_type = r#type;
    let p_start_time_at_millis = start_time_at_millis;
    let p_end_time_at_millis = end_time_at_millis;
    let p_limit = limit;
    let p_page = page;

    let uri_str = format!("{}/v1/exchange/candlesticks", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    req_builder = req_builder.query(&[("interval", &p_interval.to_string())]);
    req_builder = req_builder.query(&[("type", &p_type.to_string())]);
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

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;Vec&lt;String&gt;&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;Vec&lt;String&gt;&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCandlestickDataError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the current exchange information including available margin assets, markets, and rules.
pub async fn get_exchange_info(configuration: &configuration::Configuration, ) -> Result<models::ExchangeInfoResponse, Error<GetExchangeInfoError>> {

    let uri_str = format!("{}/v1/exchange/info", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ExchangeInfoResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ExchangeInfoResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetExchangeInfoError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve the funding rate history for a specific market address.
pub async fn get_funding_rate_history(configuration: &configuration::Configuration, symbol: &str, limit: Option<u32>, start_time_at_millis: Option<u64>, end_time_at_millis: Option<u64>, page: Option<u32>) -> Result<Vec<models::FundingRateEntry>, Error<GetFundingRateHistoryError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_limit = limit;
    let p_start_time_at_millis = start_time_at_millis;
    let p_end_time_at_millis = end_time_at_millis;
    let p_page = page;

    let uri_str = format!("{}/v1/exchange/fundingRateHistory", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_start_time_at_millis {
        req_builder = req_builder.query(&[("startTimeAtMillis", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_time_at_millis {
        req_builder = req_builder.query(&[("endTimeAtMillis", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::FundingRateEntry&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::FundingRateEntry&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFundingRateHistoryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_market_ticker(configuration: &configuration::Configuration, symbol: &str) -> Result<models::TickerResponse, Error<GetMarketTickerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;

    let uri_str = format!("{}/v1/exchange/ticker", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::TickerResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::TickerResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMarketTickerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_orderbook_depth(configuration: &configuration::Configuration, symbol: &str, limit: Option<u32>) -> Result<models::OrderbookDepthResponse, Error<GetOrderbookDepthError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_limit = limit;

    let uri_str = format!("{}/v1/exchange/depth", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::OrderbookDepthResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::OrderbookDepthResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetOrderbookDepthError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_recent_trades(configuration: &configuration::Configuration, symbol: &str, trade_type: Option<&str>, limit: Option<u32>, start_time_at_millis: Option<i64>, end_time_at_millis: Option<u64>, page: Option<u32>) -> Result<Vec<models::Trade1>, Error<GetRecentTradesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_trade_type = trade_type;
    let p_limit = limit;
    let p_start_time_at_millis = start_time_at_millis;
    let p_end_time_at_millis = end_time_at_millis;
    let p_page = page;

    let uri_str = format!("{}/v1/exchange/trades", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_symbol.to_string())]);
    if let Some(ref param_value) = p_trade_type {
        req_builder = req_builder.query(&[("tradeType", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_start_time_at_millis {
        req_builder = req_builder.query(&[("startTimeAtMillis", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_time_at_millis {
        req_builder = req_builder.query(&[("endTimeAtMillis", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::Trade1&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::Trade1&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetRecentTradesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

