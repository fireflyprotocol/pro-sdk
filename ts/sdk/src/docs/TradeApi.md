# TradeApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**cancelOrders**](#cancelorders) | **PUT** /api/v1/trade/orders/cancel | /trade/orders/cancel|
|[**cancelStandbyOrders**](#cancelstandbyorders) | **PUT** /api/v1/trade/orders/cancel/standby | /trade/orders/cancel/standby|
|[**getOpenOrders**](#getopenorders) | **GET** /api/v1/trade/openOrders | /trade/openOrders|
|[**getStandbyOrders**](#getstandbyorders) | **GET** /api/v1/trade/standbyOrders | /trade/standbyOrders|
|[**postCreateOrder**](#postcreateorder) | **POST** /api/v1/trade/orders | /trade/orders|
|[**postWithdraw**](#postwithdraw) | **POST** /api/v1/trade/withdraw | /trade/withdraw|
|[**putAdjustIsolatedMargin**](#putadjustisolatedmargin) | **PUT** /api/v1/trade/adjustIsolatedMargin | /trade/adjustIsolatedMargin|
|[**putAuthorizeAccount**](#putauthorizeaccount) | **PUT** /api/v1/trade/accounts/authorize | /trade/accounts/authorize|
|[**putDeauthorizeAccount**](#putdeauthorizeaccount) | **PUT** /api/v1/trade/accounts/deauthorize | /trade/accounts/deauthorize|
|[**putLeverageUpdate**](#putleverageupdate) | **PUT** /api/v1/trade/leverage | /trade/leverage|

# **cancelOrders**
> cancelOrders(cancelOrdersRequest)

Cancel orders for a market using order hashes. - May be a single order hash or a list of order hashes. - All orders must belong to the same account. - If no order hashes are specified, then will cancel all orders for the given market  - All orders being cancelled by request will receive the same time priority. 

### Example

```typescript
import {
    TradeApi,
    Configuration,
    CancelOrdersRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new TradeApi(configuration);

let cancelOrdersRequest: CancelOrdersRequest; //

const { status, data } = await apiInstance.cancelOrders(
    cancelOrdersRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **cancelOrdersRequest** | **CancelOrdersRequest**|  | |


### Return type

void (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**202** | Order cancellation submitted successfully. |  -  |
|**400** | Bad Request. Invalid body parameters. |  -  |
|**401** | Unauthorized. Authentication is required or invalid. |  -  |
|**403** | Forbidden. |  -  |
|**429** | Too Many Requests. The request was rate limited. |  -  |
|**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |
|**503** | Service Unavailable. The service is currently unavailable. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **cancelStandbyOrders**
> CancelOrdersResponse cancelStandbyOrders(cancelOrdersRequest)

Cancel orders in standby for a market using order hashes. - May be a single order hash or a list of order hashes. - All orders must belong to the same account. - If no order hashes are specified, then will cancel all orders for the given market - All orders being cancelled by request will receive the same time priority. 

### Example

```typescript
import {
    TradeApi,
    Configuration,
    CancelOrdersRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new TradeApi(configuration);

let cancelOrdersRequest: CancelOrdersRequest; //

const { status, data } = await apiInstance.cancelStandbyOrders(
    cancelOrdersRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **cancelOrdersRequest** | **CancelOrdersRequest**|  | |


### Return type

**CancelOrdersResponse**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Orders cancelled successfully. |  -  |
|**400** | Bad Request. Invalid body parameters. |  -  |
|**403** | Forbidden. |  -  |
|**401** | Unauthorized. Authentication is required or invalid. |  -  |
|**429** | Too Many Requests. The request was rate limited. |  -  |
|**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |
|**503** | Service Unavailable. The service is currently unavailable. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOpenOrders**
> Array<OpenOrderResponse> getOpenOrders()

Retrieve details of open orders for a specific account.

### Example

```typescript
import {
    TradeApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new TradeApi(configuration);

let symbol: string; //Filter by specific perpetual symbol (optional) (optional) (default to undefined)

const { status, data } = await apiInstance.getOpenOrders(
    symbol
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **symbol** | [**string**] | Filter by specific perpetual symbol (optional) | (optional) defaults to undefined|


### Return type

**Array<OpenOrderResponse>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Open orders retrieved successfully. |  -  |
|**400** | Bad Request. The request was invalid or malformed. |  -  |
|**401** | Unauthorized. Authentication is required or invalid. |  -  |
|**404** | Not Found. The requested account address was not found. |  -  |
|**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getStandbyOrders**
> Array<OpenOrderResponse> getStandbyOrders()

Retrieve details of orders in standby for a specific account.

### Example

```typescript
import {
    TradeApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new TradeApi(configuration);

let symbol: string; //Filter by specific perpetual symbol (optional) (optional) (default to undefined)

const { status, data } = await apiInstance.getStandbyOrders(
    symbol
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **symbol** | [**string**] | Filter by specific perpetual symbol (optional) | (optional) defaults to undefined|


### Return type

**Array<OpenOrderResponse>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Orders in standby retrieved successfully. |  -  |
|**400** | Bad Request. The request was invalid or malformed. |  -  |
|**401** | Unauthorized. Authentication is required or invalid. |  -  |
|**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postCreateOrder**
> CreateOrderResponse postCreateOrder(createOrderRequest)

Submit a new order for execution.

### Example

```typescript
import {
    TradeApi,
    Configuration,
    CreateOrderRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new TradeApi(configuration);

let createOrderRequest: CreateOrderRequest; //

const { status, data } = await apiInstance.postCreateOrder(
    createOrderRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **createOrderRequest** | **CreateOrderRequest**|  | |


### Return type

**CreateOrderResponse**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Order creation successful. |  -  |
|**202** | Order creation submitted successfully. |  * Taker-Delay - Indicates whether the order creation request was delayed (e.g. due to speed bump). <br>  |
|**400** | Bad Request. The request was invalid or malformed. |  -  |
|**401** | Unauthorized. Authentication is required or invalid. Signature is invalid |  -  |
|**403** | Forbidden. |  -  |
|**404** | Not Found. The requested account was not found. |  -  |
|**429** | Too Many Requests. The request was rate limited. |  -  |
|**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |
|**503** | Service Unavailable. The service is currently unavailable. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postWithdraw**
> postWithdraw(withdrawRequest)

Initiates a withdraw action to remove some amount of funds from a user\'s account.

### Example

```typescript
import {
    TradeApi,
    Configuration,
    WithdrawRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new TradeApi(configuration);

let withdrawRequest: WithdrawRequest; //

const { status, data } = await apiInstance.postWithdraw(
    withdrawRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **withdrawRequest** | **WithdrawRequest**|  | |


### Return type

void (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**202** | Withdraw request submitted successfully. |  -  |
|**400** | Bad Request. The request was invalid or malformed. |  -  |
|**401** | Unauthorized. Authentication is required or invalid. Signature is invalid |  -  |
|**403** | Forbidden. |  -  |
|**404** | Not Found. The requested resource was not found. |  -  |
|**429** | Too Many Requests. The request was rate limited. |  -  |
|**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |
|**503** | Service Unavailable. The service is currently unavailable. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **putAdjustIsolatedMargin**
> putAdjustIsolatedMargin(adjustIsolatedMarginRequest)

Adjust margin for an isolated position on a specific market.

### Example

```typescript
import {
    TradeApi,
    Configuration,
    AdjustIsolatedMarginRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new TradeApi(configuration);

let adjustIsolatedMarginRequest: AdjustIsolatedMarginRequest; //

const { status, data } = await apiInstance.putAdjustIsolatedMargin(
    adjustIsolatedMarginRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **adjustIsolatedMarginRequest** | **AdjustIsolatedMarginRequest**|  | |


### Return type

void (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**202** | Adjust margin request sent successfully. |  -  |
|**400** | Bad Request. The request was invalid or malformed. |  -  |
|**401** | Unauthorized. Authentication is required or invalid. Signature is invalid |  -  |
|**403** | Forbidden. |  -  |
|**404** | Not Found. The requested account was not found. |  -  |
|**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |
|**503** | Service Unavailable. The service is currently unavailable. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **putAuthorizeAccount**
> putAuthorizeAccount(accountAuthorizationRequest)

Authorizes an account to trade, perform liquidations and more, on behalf of another account.

### Example

```typescript
import {
    TradeApi,
    Configuration,
    AccountAuthorizationRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new TradeApi(configuration);

let accountAuthorizationRequest: AccountAuthorizationRequest; //

const { status, data } = await apiInstance.putAuthorizeAccount(
    accountAuthorizationRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **accountAuthorizationRequest** | **AccountAuthorizationRequest**|  | |


### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**202** | Authorize Account request submitted successfully |  -  |
|**400** | Bad Request. The request was invalid or malformed. |  -  |
|**401** | Unauthorized. Authentication is required or invalid. Signature is invalid |  -  |
|**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |
|**503** | Service Unavailable. The service is currently unavailable. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **putDeauthorizeAccount**
> putDeauthorizeAccount(accountAuthorizationRequest)

Deauthorizes an account to trade, perform liquidations and more, on behalf of another account.

### Example

```typescript
import {
    TradeApi,
    Configuration,
    AccountAuthorizationRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new TradeApi(configuration);

let accountAuthorizationRequest: AccountAuthorizationRequest; //

const { status, data } = await apiInstance.putDeauthorizeAccount(
    accountAuthorizationRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **accountAuthorizationRequest** | **AccountAuthorizationRequest**|  | |


### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**202** | Deauthorize Account request submitted successfully |  -  |
|**400** | Bad Request. The request was invalid or malformed. |  -  |
|**401** | Unauthorized. Authentication is required or invalid. Signature is invalid |  -  |
|**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |
|**503** | Service Unavailable. The service is currently unavailable. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **putLeverageUpdate**
> putLeverageUpdate(accountPositionLeverageUpdateRequest)

Updates leverage for positions of a given market and closes all open orders for that market.

### Example

```typescript
import {
    TradeApi,
    Configuration,
    AccountPositionLeverageUpdateRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new TradeApi(configuration);

let accountPositionLeverageUpdateRequest: AccountPositionLeverageUpdateRequest; //

const { status, data } = await apiInstance.putLeverageUpdate(
    accountPositionLeverageUpdateRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **accountPositionLeverageUpdateRequest** | **AccountPositionLeverageUpdateRequest**|  | |


### Return type

void (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**202** | Leverage update request sent successfully. |  -  |
|**400** | Bad Request. The request was invalid or malformed. |  -  |
|**401** | Unauthorized. Authentication is required or invalid. Signature is invalid |  -  |
|**403** | Forbidden. |  -  |
|**404** | Not Found. The requested account was not found. |  -  |
|**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |
|**503** | Service Unavailable. The service is currently unavailable. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

