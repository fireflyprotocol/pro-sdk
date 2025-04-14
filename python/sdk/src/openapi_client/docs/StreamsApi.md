# openapi_client.StreamsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**web_socket_account_data**](StreamsApi.md#web_socket_account_data) | **GET** /ws/account | WebSocket Account Streams
[**web_socket_market_data**](StreamsApi.md#web_socket_market_data) | **GET** /ws/market | WebSocket Market Streams


# **web_socket_account_data**
> web_socket_account_data(authorization, upgrade, sec_web_socket_key, sec_web_socket_version)

WebSocket Account Streams

WebSocket Account Streams URL.

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://api.sui-staging.bluefin.io
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "https://api.sui-staging.bluefin.io"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure Bearer authorization (JWT): bearerAuth
configuration = openapi_client.Configuration(
    access_token = os.environ["BEARER_TOKEN"]
)

# Enter a context with an instance of the API client
async with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.StreamsApi(api_client)
    authorization = 'Bearer JWT_token' # str | 
    upgrade = 'upgrade_example' # str | 
    sec_web_socket_key = 'sec_web_socket_key_example' # str | WebSocket key used during the handshake.
    sec_web_socket_version = 'sec_web_socket_version_example' # str | WebSocket protocol version.

    try:
        # WebSocket Account Streams
        await api_instance.web_socket_account_data(authorization, upgrade, sec_web_socket_key, sec_web_socket_version)
    except Exception as e:
        print("Exception when calling StreamsApi->web_socket_account_data: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **authorization** | **str**|  | 
 **upgrade** | **str**|  | 
 **sec_web_socket_key** | **str**| WebSocket key used during the handshake. | 
 **sec_web_socket_version** | **str**| WebSocket protocol version. | 

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
**101** | Switching Protocols - The client is switching protocols as requested by the server. |  * Upgrade - Recognized WebSocket protocol <br>  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **web_socket_market_data**
> web_socket_market_data(upgrade, sec_web_socket_key, sec_web_socket_version)

WebSocket Market Streams

WebSocket Market Streams URL.

### Example


```python
import openapi_client
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
    api_instance = openapi_client.StreamsApi(api_client)
    upgrade = 'upgrade_example' # str | 
    sec_web_socket_key = 'sec_web_socket_key_example' # str | WebSocket key used during the handshake.
    sec_web_socket_version = 'sec_web_socket_version_example' # str | WebSocket protocol version.

    try:
        # WebSocket Market Streams
        await api_instance.web_socket_market_data(upgrade, sec_web_socket_key, sec_web_socket_version)
    except Exception as e:
        print("Exception when calling StreamsApi->web_socket_market_data: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **upgrade** | **str**|  | 
 **sec_web_socket_key** | **str**| WebSocket key used during the handshake. | 
 **sec_web_socket_version** | **str**| WebSocket protocol version. | 

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
**101** | Switching Protocols - The client is switching protocols as requested by the server. |  * Upgrade - Recognized WebSocket protocol <br>  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

