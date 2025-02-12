# openapi_client.TradingGatewayApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_orders**](TradingGatewayApi.md#cancel_orders) | **PUT** /api/v1/trade/orders/cancel | Cancel orders for a market using order hashes
[**get_open_orders**](TradingGatewayApi.md#get_open_orders) | **GET** /api/v1/trade/openOrders | Get Open Orders
[**post_create_order**](TradingGatewayApi.md#post_create_order) | **POST** /api/v1/trade/orders | Create a new order
[**post_withdraw**](TradingGatewayApi.md#post_withdraw) | **POST** /api/v1/trade/withdraw | Initiate a withdraw
[**put_leverage_update**](TradingGatewayApi.md#put_leverage_update) | **PUT** /api/v1/trade/leverage | Updates leverage for positions


# **cancel_orders**
> cancel_orders(cancel_orders_request)

Cancel orders for a market using order hashes

- May be a single order hash or a list of order hashes. - All orders must belong to the same account. - If no order hashes are specified, then will cancel all orders for the given market  - All orders being cancelled by request will receive the same time priority. 

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.cancel_orders_request import CancelOrdersRequest
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:8080
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost:8080"
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
    api_instance = openapi_client.TradingGatewayApi(api_client)
    cancel_orders_request = openapi_client.CancelOrdersRequest() # CancelOrdersRequest | 

    try:
        # Cancel orders for a market using order hashes
        await api_instance.cancel_orders(cancel_orders_request)
    except Exception as e:
        print("Exception when calling TradingGatewayApi->cancel_orders: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cancel_orders_request** | [**CancelOrdersRequest**](CancelOrdersRequest.md)|  | 

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
**202** | Order cancellation submitted successfully. |  -  |
**400** | Bad Request. Invalid body parameters. |  -  |
**401** | Unauthorized. Authentication is required or invalid. |  -  |
**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_open_orders**
> List[OpenOrderResponse] get_open_orders(symbol=symbol)

Get Open Orders

Retrieve details of open orders for a specific account.

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.open_order_response import OpenOrderResponse
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:8080
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost:8080"
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
    api_instance = openapi_client.TradingGatewayApi(api_client)
    symbol = 'ETH-PERP' # str | Filter by specific perpetual symbol (optional) (optional)

    try:
        # Get Open Orders
        api_response = await api_instance.get_open_orders(symbol=symbol)
        print("The response of TradingGatewayApi->get_open_orders:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling TradingGatewayApi->get_open_orders: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **str**| Filter by specific perpetual symbol (optional) | [optional] 

### Return type

[**List[OpenOrderResponse]**](OpenOrderResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Open orders retrieved successfully. |  -  |
**400** | Bad Request. The request was invalid or malformed. |  -  |
**401** | Unauthorized. Authentication is required or invalid. |  -  |
**404** | Not Found. The requested account address was not found. |  -  |
**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_create_order**
> PostCreateOrder202Response post_create_order(create_order_request)

Create a new order

Submit a new order for execution.

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.create_order_request import CreateOrderRequest
from openapi_client.models.post_create_order202_response import PostCreateOrder202Response
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:8080
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost:8080"
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
    api_instance = openapi_client.TradingGatewayApi(api_client)
    create_order_request = openapi_client.CreateOrderRequest() # CreateOrderRequest | 

    try:
        # Create a new order
        api_response = await api_instance.post_create_order(create_order_request)
        print("The response of TradingGatewayApi->post_create_order:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling TradingGatewayApi->post_create_order: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_order_request** | [**CreateOrderRequest**](CreateOrderRequest.md)|  | 

### Return type

[**PostCreateOrder202Response**](PostCreateOrder202Response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**202** | Order creation submitted successfully. |  -  |
**400** | Bad Request. The request was invalid or malformed. |  -  |
**401** | Unauthorized. Authentication is required or invalid. Signature is invalid |  -  |
**404** | Not Found. The requested account was not found. |  -  |
**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_withdraw**
> post_withdraw(withdraw_request)

Initiate a withdraw

Initiates a withdraw action to withdraw some amount of assets from a user's account

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.withdraw_request import WithdrawRequest
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:8080
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost:8080"
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
    api_instance = openapi_client.TradingGatewayApi(api_client)
    withdraw_request = openapi_client.WithdrawRequest() # WithdrawRequest | 

    try:
        # Initiate a withdraw
        await api_instance.post_withdraw(withdraw_request)
    except Exception as e:
        print("Exception when calling TradingGatewayApi->post_withdraw: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **withdraw_request** | [**WithdrawRequest**](WithdrawRequest.md)|  | 

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
**202** | Withdraw request submitted successfully. |  -  |
**400** | Bad Request. The request was invalid or malformed. |  -  |
**401** | Unauthorized. Authentication is required or invalid. Signature is invalid |  -  |
**404** | Not Found. The requested resource was not found. |  -  |
**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_leverage_update**
> put_leverage_update(account_position_leverage_update_request)

Updates leverage for positions

Updates leverage for positions of a given market, closes all open orders for that market

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.account_position_leverage_update_request import AccountPositionLeverageUpdateRequest
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:8080
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost:8080"
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
    api_instance = openapi_client.TradingGatewayApi(api_client)
    account_position_leverage_update_request = openapi_client.AccountPositionLeverageUpdateRequest() # AccountPositionLeverageUpdateRequest | 

    try:
        # Updates leverage for positions
        await api_instance.put_leverage_update(account_position_leverage_update_request)
    except Exception as e:
        print("Exception when calling TradingGatewayApi->put_leverage_update: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_position_leverage_update_request** | [**AccountPositionLeverageUpdateRequest**](AccountPositionLeverageUpdateRequest.md)|  | 

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
**202** | Leverage update request sent successfully. |  -  |
**400** | Bad Request. The request was invalid or malformed. |  -  |
**401** | Unauthorized. Authentication is required or invalid. Signature is invalid |  -  |
**404** | Not Found. The requested account was not found. |  -  |
**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

