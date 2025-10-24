# openapi_client.ExchangeApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_market_ticker**](ExchangeApi.md#get_all_market_ticker) | **GET** /v1/exchange/tickers | /exchange/tickers
[**get_candlestick_data**](ExchangeApi.md#get_candlestick_data) | **GET** /v1/exchange/candlesticks | /exchange/candlesticks
[**get_country**](ExchangeApi.md#get_country) | **GET** /v1/exchange/country | /exchange/country
[**get_exchange_info**](ExchangeApi.md#get_exchange_info) | **GET** /v1/exchange/info | /exchange/info
[**get_exchange_stats**](ExchangeApi.md#get_exchange_stats) | **GET** /v1/exchange/stats | /exchange/stats
[**get_exchange_stats_all_time**](ExchangeApi.md#get_exchange_stats_all_time) | **GET** /v1/exchange/stats/allTime | /v1/exchange/stats/allTime
[**get_funding_rate_history**](ExchangeApi.md#get_funding_rate_history) | **GET** /v1/exchange/fundingRateHistory | /exchange/fundingRateHistory
[**get_market_ticker**](ExchangeApi.md#get_market_ticker) | **GET** /v1/exchange/ticker | /exchange/ticker
[**get_orderbook_depth**](ExchangeApi.md#get_orderbook_depth) | **GET** /v1/exchange/depth | /exchange/depth
[**get_recent_trades**](ExchangeApi.md#get_recent_trades) | **GET** /v1/exchange/trades | /exchange/trades


# **get_all_market_ticker**
> List[TickerResponse] get_all_market_ticker()

/exchange/tickers

Retrieves all market ticker information.

### Example


```python
import openapi_client
from openapi_client.models.ticker_response import TickerResponse
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://api.sui-staging.bluefin.io
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "https://api.sui-staging.bluefin.io"
)


# Enter a context with an instance of the API client
async with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.ExchangeApi(api_client)

    try:
        # /exchange/tickers
        api_response = await api_instance.get_all_market_ticker()
        print("The response of ExchangeApi->get_all_market_ticker:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ExchangeApi->get_all_market_ticker: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**List[TickerResponse]**](TickerResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | OK |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_candlestick_data**
> List[List[str]] get_candlestick_data(symbol, interval, type, start_time_at_millis=start_time_at_millis, end_time_at_millis=end_time_at_millis, limit=limit, page=page)

/exchange/candlesticks

Retrieves candle stick data for a market.

### Example


```python
import openapi_client
from openapi_client.models.candle_price_type import CandlePriceType
from openapi_client.models.kline_interval import KlineInterval
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://api.sui-staging.bluefin.io
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "https://api.sui-staging.bluefin.io"
)


# Enter a context with an instance of the API client
async with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.ExchangeApi(api_client)
    symbol = '291' # str | The market symbol to get the klines for.
    interval = openapi_client.KlineInterval() # KlineInterval | The interval to get the klines for.
    type = openapi_client.CandlePriceType() # CandlePriceType | Candle price type (last price, market price or oracle).
    start_time_at_millis = 1499040000000 # int | Timestamp in milliseconds in ms to get klines from. (optional)
    end_time_at_millis = 1499644799999 # int | Timestamp in milliseconds in ms to get klines until. (optional)
    limit = 50 # int | Default 50; max 1000. (optional) (default to 50)
    page = 56 # int | The page number to retrieve in a paginated response. (optional)

    try:
        # /exchange/candlesticks
        api_response = await api_instance.get_candlestick_data(symbol, interval, type, start_time_at_millis=start_time_at_millis, end_time_at_millis=end_time_at_millis, limit=limit, page=page)
        print("The response of ExchangeApi->get_candlestick_data:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ExchangeApi->get_candlestick_data: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **str**| The market symbol to get the klines for. | 
 **interval** | [**KlineInterval**](.md)| The interval to get the klines for. | 
 **type** | [**CandlePriceType**](.md)| Candle price type (last price, market price or oracle). | 
 **start_time_at_millis** | **int**| Timestamp in milliseconds in ms to get klines from. | [optional] 
 **end_time_at_millis** | **int**| Timestamp in milliseconds in ms to get klines until. | [optional] 
 **limit** | **int**| Default 50; max 1000. | [optional] [default to 50]
 **page** | **int**| The page number to retrieve in a paginated response. | [optional] 

### Return type

**List[List[str]]**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Kline/candlestick data |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_country**
> CountryResponse get_country()

/exchange/country

Check if the country is geo restricted.

### Example


```python
import openapi_client
from openapi_client.models.country_response import CountryResponse
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://api.sui-staging.bluefin.io
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "https://api.sui-staging.bluefin.io"
)


# Enter a context with an instance of the API client
async with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.ExchangeApi(api_client)

    try:
        # /exchange/country
        api_response = await api_instance.get_country()
        print("The response of ExchangeApi->get_country:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ExchangeApi->get_country: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**CountryResponse**](CountryResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response |  -  |
**500** | Internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_exchange_info**
> ExchangeInfoResponse get_exchange_info()

/exchange/info

Returns the current exchange information including available margin assets, markets, and rules.

### Example


```python
import openapi_client
from openapi_client.models.exchange_info_response import ExchangeInfoResponse
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://api.sui-staging.bluefin.io
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "https://api.sui-staging.bluefin.io"
)


# Enter a context with an instance of the API client
async with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.ExchangeApi(api_client)

    try:
        # /exchange/info
        api_response = await api_instance.get_exchange_info()
        print("The response of ExchangeApi->get_exchange_info:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ExchangeApi->get_exchange_info: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**ExchangeInfoResponse**](ExchangeInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_exchange_stats**
> StatsResponse get_exchange_stats(interval=interval, start_time_at_millis=start_time_at_millis, end_time_at_millis=end_time_at_millis, limit=limit, page=page)

/exchange/stats

Retrieves exchange statistics.

### Example


```python
import openapi_client
from openapi_client.models.stats_interval import StatsInterval
from openapi_client.models.stats_response import StatsResponse
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://api.sui-staging.bluefin.io
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "https://api.sui-staging.bluefin.io"
)


# Enter a context with an instance of the API client
async with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.ExchangeApi(api_client)
    interval = openapi_client.StatsInterval() # StatsInterval |  (optional)
    start_time_at_millis = 1499040000000 # int | Timestamp in milliseconds. (optional)
    end_time_at_millis = 1499644799999 # int | Timestamp in milliseconds. (optional)
    limit = 30 # int | Number of records to return. Default is 30; max is 200. (optional) (default to 30)
    page = 56 # int | The page number to retrieve in a paginated response. (optional)

    try:
        # /exchange/stats
        api_response = await api_instance.get_exchange_stats(interval=interval, start_time_at_millis=start_time_at_millis, end_time_at_millis=end_time_at_millis, limit=limit, page=page)
        print("The response of ExchangeApi->get_exchange_stats:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ExchangeApi->get_exchange_stats: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **interval** | [**StatsInterval**](.md)|  | [optional] 
 **start_time_at_millis** | **int**| Timestamp in milliseconds. | [optional] 
 **end_time_at_millis** | **int**| Timestamp in milliseconds. | [optional] 
 **limit** | **int**| Number of records to return. Default is 30; max is 200. | [optional] [default to 30]
 **page** | **int**| The page number to retrieve in a paginated response. | [optional] 

### Return type

[**StatsResponse**](StatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response with exchange statistics. |  -  |
**400** | Invalid parameters provided. |  -  |
**422** | Unprocessable parameters provided. |  -  |
**500** | Internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_exchange_stats_all_time**
> StatsAllTimeResponse get_exchange_stats_all_time()

/v1/exchange/stats/allTime

Retrieves all time exchange statistics.

### Example


```python
import openapi_client
from openapi_client.models.stats_all_time_response import StatsAllTimeResponse
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://api.sui-staging.bluefin.io
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "https://api.sui-staging.bluefin.io"
)


# Enter a context with an instance of the API client
async with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.ExchangeApi(api_client)

    try:
        # /v1/exchange/stats/allTime
        api_response = await api_instance.get_exchange_stats_all_time()
        print("The response of ExchangeApi->get_exchange_stats_all_time:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ExchangeApi->get_exchange_stats_all_time: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**StatsAllTimeResponse**](StatsAllTimeResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response with exchange statistics. |  -  |
**500** | Internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_funding_rate_history**
> List[FundingRateEntry] get_funding_rate_history(symbol, limit=limit, start_time_at_millis=start_time_at_millis, end_time_at_millis=end_time_at_millis, page=page)

/exchange/fundingRateHistory

Retrieve the funding rate history for a specific market address.

### Example


```python
import openapi_client
from openapi_client.models.funding_rate_entry import FundingRateEntry
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://api.sui-staging.bluefin.io
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "https://api.sui-staging.bluefin.io"
)


# Enter a context with an instance of the API client
async with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.ExchangeApi(api_client)
    symbol = '291' # str | The market symbol to get funding rate history for
    limit = 100 # int | Number of records to return. Default is 100; max is 1000. (optional) (default to 100)
    start_time_at_millis = 56 # int | The timestamp specifies the earliest point in time for which data should be returned. The value is not included. (optional)
    end_time_at_millis = 56 # int | The timestamp specifies the latest point in time for which data should be returned. The value is included. (optional)
    page = 56 # int | The page number to retrieve in a paginated response. (optional)

    try:
        # /exchange/fundingRateHistory
        api_response = await api_instance.get_funding_rate_history(symbol, limit=limit, start_time_at_millis=start_time_at_millis, end_time_at_millis=end_time_at_millis, page=page)
        print("The response of ExchangeApi->get_funding_rate_history:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ExchangeApi->get_funding_rate_history: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **str**| The market symbol to get funding rate history for | 
 **limit** | **int**| Number of records to return. Default is 100; max is 1000. | [optional] [default to 100]
 **start_time_at_millis** | **int**| The timestamp specifies the earliest point in time for which data should be returned. The value is not included. | [optional] 
 **end_time_at_millis** | **int**| The timestamp specifies the latest point in time for which data should be returned. The value is included. | [optional] 
 **page** | **int**| The page number to retrieve in a paginated response. | [optional] 

### Return type

[**List[FundingRateEntry]**](FundingRateEntry.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response with funding rate history. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_market_ticker**
> TickerResponse get_market_ticker(symbol)

/exchange/ticker

Retrieves aggregated ticker data for a market.

### Example


```python
import openapi_client
from openapi_client.models.ticker_response import TickerResponse
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://api.sui-staging.bluefin.io
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "https://api.sui-staging.bluefin.io"
)


# Enter a context with an instance of the API client
async with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.ExchangeApi(api_client)
    symbol = '1193046' # str | Market symbol.

    try:
        # /exchange/ticker
        api_response = await api_instance.get_market_ticker(symbol)
        print("The response of ExchangeApi->get_market_ticker:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ExchangeApi->get_market_ticker: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **str**| Market symbol. | 

### Return type

[**TickerResponse**](TickerResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | OK |  -  |
**404** | Market not found. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_orderbook_depth**
> OrderbookDepthResponse get_orderbook_depth(symbol, limit=limit)

/exchange/depth

Returns the current state of the orderbook.

### Example


```python
import openapi_client
from openapi_client.models.orderbook_depth_response import OrderbookDepthResponse
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://api.sui-staging.bluefin.io
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "https://api.sui-staging.bluefin.io"
)


# Enter a context with an instance of the API client
async with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.ExchangeApi(api_client)
    symbol = '291' # str | Market symbol to get the orderbook depth for.
    limit = 500 # int | Maximum number of bids and asks to return. Default 500; max 1000. (optional) (default to 500)

    try:
        # /exchange/depth
        api_response = await api_instance.get_orderbook_depth(symbol, limit=limit)
        print("The response of ExchangeApi->get_orderbook_depth:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ExchangeApi->get_orderbook_depth: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **str**| Market symbol to get the orderbook depth for. | 
 **limit** | **int**| Maximum number of bids and asks to return. Default 500; max 1000. | [optional] [default to 500]

### Return type

[**OrderbookDepthResponse**](OrderbookDepthResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Orderbook depth |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_recent_trades**
> List[Trade] get_recent_trades(symbol, trade_type=trade_type, limit=limit, start_time_at_millis=start_time_at_millis, end_time_at_millis=end_time_at_millis, page=page)

/exchange/trades

Retrieves recent trades executed on a market.

### Example


```python
import openapi_client
from openapi_client.models.trade import Trade
from openapi_client.models.trade_type import TradeType
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://api.sui-staging.bluefin.io
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "https://api.sui-staging.bluefin.io"
)


# Enter a context with an instance of the API client
async with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.ExchangeApi(api_client)
    symbol = '291' # str | The market symbol to get the trades for.
    trade_type = openapi_client.TradeType() # TradeType | Type of trade. (optional)
    limit = 500 # int | Default 500; max 1000. (optional) (default to 500)
    start_time_at_millis = 56 # int | The timestamp specifies the earliest point in time for which data should be returned. The value is not included. (optional)
    end_time_at_millis = 56 # int | The timestamp specifies the latest point in time for which data should be returned. The value is included. (optional)
    page = 56 # int | The page number to retrieve in a paginated response. (optional)

    try:
        # /exchange/trades
        api_response = await api_instance.get_recent_trades(symbol, trade_type=trade_type, limit=limit, start_time_at_millis=start_time_at_millis, end_time_at_millis=end_time_at_millis, page=page)
        print("The response of ExchangeApi->get_recent_trades:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ExchangeApi->get_recent_trades: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **str**| The market symbol to get the trades for. | 
 **trade_type** | [**TradeType**](.md)| Type of trade. | [optional] 
 **limit** | **int**| Default 500; max 1000. | [optional] [default to 500]
 **start_time_at_millis** | **int**| The timestamp specifies the earliest point in time for which data should be returned. The value is not included. | [optional] 
 **end_time_at_millis** | **int**| The timestamp specifies the latest point in time for which data should be returned. The value is included. | [optional] 
 **page** | **int**| The page number to retrieve in a paginated response. | [optional] 

### Return type

[**List[Trade]**](Trade.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Recent trades |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

