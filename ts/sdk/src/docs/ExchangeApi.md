# ExchangeApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**getAllMarketTicker**](#getallmarketticker) | **GET** /v1/exchange/tickers | /exchange/tickers|
|[**getCandlestickData**](#getcandlestickdata) | **GET** /v1/exchange/candlesticks | /exchange/candlesticks|
|[**getExchangeInfo**](#getexchangeinfo) | **GET** /v1/exchange/info | /exchange/info|
|[**getExchangeStats**](#getexchangestats) | **GET** /v1/exchange/stats | /exchange/stats|
|[**getFundingRateHistory**](#getfundingratehistory) | **GET** /v1/exchange/fundingRateHistory | /exchange/fundingRateHistory|
|[**getMarketTicker**](#getmarketticker) | **GET** /v1/exchange/ticker | /exchange/ticker|
|[**getOrderbookDepth**](#getorderbookdepth) | **GET** /v1/exchange/depth | /exchange/depth|
|[**getRecentTrades**](#getrecenttrades) | **GET** /v1/exchange/trades | /exchange/trades|

# **getAllMarketTicker**
> Array<TickerResponse> getAllMarketTicker()

Retrieves all market ticker information.

### Example

```typescript
import {
    ExchangeApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new ExchangeApi(configuration);

const { status, data } = await apiInstance.getAllMarketTicker();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<TickerResponse>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | OK |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getCandlestickData**
> Array<Array<string>> getCandlestickData()

Retrieves candle stick data for a market.

### Example

```typescript
import {
    ExchangeApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new ExchangeApi(configuration);

let symbol: string; //The market symbol to get the klines for. (default to undefined)
let interval: KlineInterval; //The interval to get the klines for. (default to undefined)
let type: CandlePriceType; //Candle price type (last price, market price or oracle). (default to undefined)
let startTimeAtMillis: number; //Timestamp in milliseconds in ms to get klines from. (optional) (default to undefined)
let endTimeAtMillis: number; //Timestamp in milliseconds in ms to get klines until. (optional) (default to undefined)
let limit: number; //Default 50; max 1000. (optional) (default to 50)
let page: number; //The page number to retrieve in a paginated response. (optional) (default to undefined)

const { status, data } = await apiInstance.getCandlestickData(
    symbol,
    interval,
    type,
    startTimeAtMillis,
    endTimeAtMillis,
    limit,
    page
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **symbol** | [**string**] | The market symbol to get the klines for. | defaults to undefined|
| **interval** | **KlineInterval** | The interval to get the klines for. | defaults to undefined|
| **type** | **CandlePriceType** | Candle price type (last price, market price or oracle). | defaults to undefined|
| **startTimeAtMillis** | [**number**] | Timestamp in milliseconds in ms to get klines from. | (optional) defaults to undefined|
| **endTimeAtMillis** | [**number**] | Timestamp in milliseconds in ms to get klines until. | (optional) defaults to undefined|
| **limit** | [**number**] | Default 50; max 1000. | (optional) defaults to 50|
| **page** | [**number**] | The page number to retrieve in a paginated response. | (optional) defaults to undefined|


### Return type

**Array<Array<string>>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Kline/candlestick data |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getExchangeInfo**
> ExchangeInfoResponse getExchangeInfo()

Returns the current exchange information including available margin assets, markets, and rules.

### Example

```typescript
import {
    ExchangeApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new ExchangeApi(configuration);

const { status, data } = await apiInstance.getExchangeInfo();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**ExchangeInfoResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getExchangeStats**
> StatsResponse getExchangeStats()

Retrieves exchange statistics.

### Example

```typescript
import {
    ExchangeApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new ExchangeApi(configuration);

let interval: StatsInterval; // (optional) (default to undefined)
let startTimeAtMillis: number; //Timestamp in milliseconds. (optional) (default to undefined)
let endTimeAtMillis: number; //Timestamp in milliseconds. (optional) (default to undefined)
let limit: number; //Number of records to return. Default is 30; max is 200. (optional) (default to 30)
let page: number; //The page number to retrieve in a paginated response. (optional) (default to undefined)

const { status, data } = await apiInstance.getExchangeStats(
    interval,
    startTimeAtMillis,
    endTimeAtMillis,
    limit,
    page
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **interval** | **StatsInterval** |  | (optional) defaults to undefined|
| **startTimeAtMillis** | [**number**] | Timestamp in milliseconds. | (optional) defaults to undefined|
| **endTimeAtMillis** | [**number**] | Timestamp in milliseconds. | (optional) defaults to undefined|
| **limit** | [**number**] | Number of records to return. Default is 30; max is 200. | (optional) defaults to 30|
| **page** | [**number**] | The page number to retrieve in a paginated response. | (optional) defaults to undefined|


### Return type

**StatsResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response with exchange statistics. |  -  |
|**400** | Invalid parameters provided. |  -  |
|**422** | Unprocessable parameters provided. |  -  |
|**500** | Internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getFundingRateHistory**
> Array<FundingRateEntry> getFundingRateHistory()

Retrieve the funding rate history for a specific market address.

### Example

```typescript
import {
    ExchangeApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new ExchangeApi(configuration);

let symbol: string; //The market symbol to get funding rate history for (default to undefined)
let limit: number; //Number of records to return. Default is 100; max is 1000. (optional) (default to 100)
let startTimeAtMillis: number; //The timestamp specifies the earliest point in time for which data should be returned. The value is not included. (optional) (default to undefined)
let endTimeAtMillis: number; //The timestamp specifies the latest point in time for which data should be returned. The value is included. (optional) (default to undefined)
let page: number; //The page number to retrieve in a paginated response. (optional) (default to undefined)

const { status, data } = await apiInstance.getFundingRateHistory(
    symbol,
    limit,
    startTimeAtMillis,
    endTimeAtMillis,
    page
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **symbol** | [**string**] | The market symbol to get funding rate history for | defaults to undefined|
| **limit** | [**number**] | Number of records to return. Default is 100; max is 1000. | (optional) defaults to 100|
| **startTimeAtMillis** | [**number**] | The timestamp specifies the earliest point in time for which data should be returned. The value is not included. | (optional) defaults to undefined|
| **endTimeAtMillis** | [**number**] | The timestamp specifies the latest point in time for which data should be returned. The value is included. | (optional) defaults to undefined|
| **page** | [**number**] | The page number to retrieve in a paginated response. | (optional) defaults to undefined|


### Return type

**Array<FundingRateEntry>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response with funding rate history. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getMarketTicker**
> TickerResponse getMarketTicker()

Retrieves aggregated ticker data for a market.

### Example

```typescript
import {
    ExchangeApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new ExchangeApi(configuration);

let symbol: string; //Market symbol. (default to undefined)

const { status, data } = await apiInstance.getMarketTicker(
    symbol
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **symbol** | [**string**] | Market symbol. | defaults to undefined|


### Return type

**TickerResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | OK |  -  |
|**404** | Market not found. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOrderbookDepth**
> OrderbookDepthResponse getOrderbookDepth()

Returns the current state of the orderbook.

### Example

```typescript
import {
    ExchangeApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new ExchangeApi(configuration);

let symbol: string; //Market symbol to get the orderbook depth for. (default to undefined)
let limit: number; //Maximum number of bids and asks to return. Default 500; max 1000. (optional) (default to 500)

const { status, data } = await apiInstance.getOrderbookDepth(
    symbol,
    limit
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **symbol** | [**string**] | Market symbol to get the orderbook depth for. | defaults to undefined|
| **limit** | [**number**] | Maximum number of bids and asks to return. Default 500; max 1000. | (optional) defaults to 500|


### Return type

**OrderbookDepthResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Orderbook depth |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRecentTrades**
> Array<Trade> getRecentTrades()

Retrieves recent trades executed on a market.

### Example

```typescript
import {
    ExchangeApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new ExchangeApi(configuration);

let symbol: string; //The market symbol to get the trades for. (default to undefined)
let tradeType: TradeType; //Type of trade. (optional) (default to undefined)
let limit: number; //Default 500; max 1000. (optional) (default to 500)
let startTimeAtMillis: number; //The timestamp specifies the earliest point in time for which data should be returned. The value is not included. (optional) (default to undefined)
let endTimeAtMillis: number; //The timestamp specifies the latest point in time for which data should be returned. The value is included. (optional) (default to undefined)
let page: number; //The page number to retrieve in a paginated response. (optional) (default to undefined)

const { status, data } = await apiInstance.getRecentTrades(
    symbol,
    tradeType,
    limit,
    startTimeAtMillis,
    endTimeAtMillis,
    page
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **symbol** | [**string**] | The market symbol to get the trades for. | defaults to undefined|
| **tradeType** | **TradeType** | Type of trade. | (optional) defaults to undefined|
| **limit** | [**number**] | Default 500; max 1000. | (optional) defaults to 500|
| **startTimeAtMillis** | [**number**] | The timestamp specifies the earliest point in time for which data should be returned. The value is not included. | (optional) defaults to undefined|
| **endTimeAtMillis** | [**number**] | The timestamp specifies the latest point in time for which data should be returned. The value is included. | (optional) defaults to undefined|
| **page** | [**number**] | The page number to retrieve in a paginated response. | (optional) defaults to undefined|


### Return type

**Array<Trade>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Recent trades |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

