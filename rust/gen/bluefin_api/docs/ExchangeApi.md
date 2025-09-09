# \ExchangeApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_market_ticker**](ExchangeApi.md#get_all_market_ticker) | **GET** /v1/exchange/tickers | /exchange/tickers
[**get_candlestick_data**](ExchangeApi.md#get_candlestick_data) | **GET** /v1/exchange/candlesticks | /exchange/candlesticks
[**get_country**](ExchangeApi.md#get_country) | **GET** /v1/exchange/country | /exchange/country
[**get_exchange_info**](ExchangeApi.md#get_exchange_info) | **GET** /v1/exchange/info | /exchange/info
[**get_exchange_stats**](ExchangeApi.md#get_exchange_stats) | **GET** /v1/exchange/stats | /exchange/stats
[**get_funding_rate_history**](ExchangeApi.md#get_funding_rate_history) | **GET** /v1/exchange/fundingRateHistory | /exchange/fundingRateHistory
[**get_market_ticker**](ExchangeApi.md#get_market_ticker) | **GET** /v1/exchange/ticker | /exchange/ticker
[**get_orderbook_depth**](ExchangeApi.md#get_orderbook_depth) | **GET** /v1/exchange/depth | /exchange/depth
[**get_recent_trades**](ExchangeApi.md#get_recent_trades) | **GET** /v1/exchange/trades | /exchange/trades



## get_all_market_ticker

> Vec<models::TickerResponse> get_all_market_ticker()
/exchange/tickers

Retrieves all market ticker information.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TickerResponse>**](TickerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_candlestick_data

> Vec<Vec<String>> get_candlestick_data(symbol, interval, r#type, start_time_at_millis, end_time_at_millis, limit, page)
/exchange/candlesticks

Retrieves candle stick data for a market.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | The market symbol to get the klines for. | [required] |
**interval** | [**KlineInterval**](.md) | The interval to get the klines for. | [required] |
**r#type** | [**CandlePriceType**](.md) | Candle price type (last price, market price or oracle). | [required] |
**start_time_at_millis** | Option<**u64**> | Timestamp in milliseconds in ms to get klines from. |  |
**end_time_at_millis** | Option<**u64**> | Timestamp in milliseconds in ms to get klines until. |  |
**limit** | Option<**u32**> | Default 50; max 1000. |  |[default to 50]
**page** | Option<**u32**> | The page number to retrieve in a paginated response. |  |

### Return type

[**Vec<Vec<String>>**](Vec.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_country

> models::CountryResponse get_country()
/exchange/country

Check if the country is geo restricted.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CountryResponse**](CountryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exchange_info

> models::ExchangeInfoResponse get_exchange_info()
/exchange/info

Returns the current exchange information including available margin assets, markets, and rules.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ExchangeInfoResponse**](ExchangeInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exchange_stats

> models::StatsResponse get_exchange_stats(interval, start_time_at_millis, end_time_at_millis, limit, page)
/exchange/stats

Retrieves exchange statistics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval** | Option<[**StatsInterval**](.md)> |  |  |
**start_time_at_millis** | Option<**u64**> | Timestamp in milliseconds. |  |
**end_time_at_millis** | Option<**u64**> | Timestamp in milliseconds. |  |
**limit** | Option<**u32**> | Number of records to return. Default is 30; max is 200. |  |[default to 30]
**page** | Option<**u32**> | The page number to retrieve in a paginated response. |  |

### Return type

[**models::StatsResponse**](StatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_funding_rate_history

> Vec<models::FundingRateEntry> get_funding_rate_history(symbol, limit, start_time_at_millis, end_time_at_millis, page)
/exchange/fundingRateHistory

Retrieve the funding rate history for a specific market address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | The market symbol to get funding rate history for | [required] |
**limit** | Option<**u32**> | Number of records to return. Default is 100; max is 1000. |  |[default to 100]
**start_time_at_millis** | Option<**u64**> | The timestamp specifies the earliest point in time for which data should be returned. The value is not included. |  |
**end_time_at_millis** | Option<**u64**> | The timestamp specifies the latest point in time for which data should be returned. The value is included. |  |
**page** | Option<**u32**> | The page number to retrieve in a paginated response. |  |

### Return type

[**Vec<models::FundingRateEntry>**](FundingRateEntry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_market_ticker

> models::TickerResponse get_market_ticker(symbol)
/exchange/ticker

Retrieves aggregated ticker data for a market.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Market symbol. | [required] |

### Return type

[**models::TickerResponse**](TickerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orderbook_depth

> models::OrderbookDepthResponse get_orderbook_depth(symbol, limit)
/exchange/depth

Returns the current state of the orderbook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Market symbol to get the orderbook depth for. | [required] |
**limit** | Option<**u32**> | Maximum number of bids and asks to return. Default 500; max 1000. |  |[default to 500]

### Return type

[**models::OrderbookDepthResponse**](OrderbookDepthResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recent_trades

> Vec<models::Trade> get_recent_trades(symbol, trade_type, limit, start_time_at_millis, end_time_at_millis, page)
/exchange/trades

Retrieves recent trades executed on a market.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | The market symbol to get the trades for. | [required] |
**trade_type** | Option<[**TradeType**](.md)> | Type of trade. |  |
**limit** | Option<**u32**> | Default 500; max 1000. |  |[default to 500]
**start_time_at_millis** | Option<**i64**> | The timestamp specifies the earliest point in time for which data should be returned. The value is not included. |  |
**end_time_at_millis** | Option<**u64**> | The timestamp specifies the latest point in time for which data should be returned. The value is included. |  |
**page** | Option<**u32**> | The page number to retrieve in a paginated response. |  |

### Return type

[**Vec<models::Trade>**](Trade.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

