# StreamsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**webSocketAccountData**](#websocketaccountdata) | **GET** /ws/account | WebSocket Account Streams|
|[**webSocketMarketData**](#websocketmarketdata) | **GET** /ws/market | WebSocket Market Streams|

# **webSocketAccountData**
> webSocketAccountData()

WebSocket Account Streams URL.

### Example

```typescript
import {
    StreamsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new StreamsApi(configuration);

let authorization: string; // (default to undefined)
let upgrade: 'websocket'; // (default to undefined)
let secWebSocketKey: string; //WebSocket key used during the handshake. (default to undefined)
let secWebSocketVersion: '13'; //WebSocket protocol version. (default to undefined)

const { status, data } = await apiInstance.webSocketAccountData(
    authorization,
    upgrade,
    secWebSocketKey,
    secWebSocketVersion
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **authorization** | [**string**] |  | defaults to undefined|
| **upgrade** | [**&#39;websocket&#39;**]**Array<&#39;websocket&#39;>** |  | defaults to undefined|
| **secWebSocketKey** | [**string**] | WebSocket key used during the handshake. | defaults to undefined|
| **secWebSocketVersion** | [**&#39;13&#39;**]**Array<&#39;13&#39;>** | WebSocket protocol version. | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**101** | Switching Protocols - The client is switching protocols as requested by the server. |  * Upgrade - Recognized WebSocket protocol <br>  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **webSocketMarketData**
> webSocketMarketData()

WebSocket Market Streams URL.

### Example

```typescript
import {
    StreamsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new StreamsApi(configuration);

let upgrade: 'websocket'; // (default to undefined)
let secWebSocketKey: string; //WebSocket key used during the handshake. (default to undefined)
let secWebSocketVersion: '13'; //WebSocket protocol version. (default to undefined)

const { status, data } = await apiInstance.webSocketMarketData(
    upgrade,
    secWebSocketKey,
    secWebSocketVersion
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **upgrade** | [**&#39;websocket&#39;**]**Array<&#39;websocket&#39;>** |  | defaults to undefined|
| **secWebSocketKey** | [**string**] | WebSocket key used during the handshake. | defaults to undefined|
| **secWebSocketVersion** | [**&#39;13&#39;**]**Array<&#39;13&#39;>** | WebSocket protocol version. | defaults to undefined|


### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**101** | Switching Protocols - The client is switching protocols as requested by the server. |  * Upgrade - Recognized WebSocket protocol <br>  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

