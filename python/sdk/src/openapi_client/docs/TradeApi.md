# openapi_client.TradeApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_orders**](TradeApi.md#cancel_orders) | **PUT** /api/v1/trade/orders/cancel | Cancel orders for a market using order hashes
[**get_open_orders**](TradeApi.md#get_open_orders) | **GET** /api/v1/trade/openOrders | Get Open Orders
[**post_create_order**](TradeApi.md#post_create_order) | **POST** /api/v1/trade/orders | Create a new order
[**post_withdraw**](TradeApi.md#post_withdraw) | **POST** /api/v1/trade/withdraw | Initiate a withdraw
[**put_adjust_isolated_margin**](TradeApi.md#put_adjust_isolated_margin) | **PUT** /api/v1/trade/adjustIsolatedMargin | Adjust margin for an isolated position for a symbol
[**put_authorize_account**](TradeApi.md#put_authorize_account) | **PUT** /api/v1/trade/accounts/authorize | Authorizes an account
[**put_deauthorize_account**](TradeApi.md#put_deauthorize_account) | **PUT** /api/v1/trade/accounts/deauthorize | Deauthorizes an account
[**put_leverage_update**](TradeApi.md#put_leverage_update) | **PUT** /api/v1/trade/leverage | Updates leverage for positions


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
    api_instance = openapi_client.TradeApi(api_client)
    cancel_orders_request = openapi_client.CancelOrdersRequest() # CancelOrdersRequest | 

    try:
        # Cancel orders for a market using order hashes
        await api_instance.cancel_orders(cancel_orders_request)
    except Exception as e:
        print("Exception when calling TradeApi->cancel_orders: %s\n" % e)
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
    api_instance = openapi_client.TradeApi(api_client)
    symbol = 'ETH-PERP' # str | Filter by specific perpetual symbol (optional) (optional)

    try:
        # Get Open Orders
        api_response = await api_instance.get_open_orders(symbol=symbol)
        print("The response of TradeApi->get_open_orders:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling TradeApi->get_open_orders: %s\n" % e)
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
    api_instance = openapi_client.TradeApi(api_client)
    create_order_request = openapi_client.CreateOrderRequest() # CreateOrderRequest | 

    try:
        # Create a new order
        api_response = await api_instance.post_create_order(create_order_request)
        print("The response of TradeApi->post_create_order:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling TradeApi->post_create_order: %s\n" % e)
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
    api_instance = openapi_client.TradeApi(api_client)
    withdraw_request = openapi_client.WithdrawRequest() # WithdrawRequest | 

    try:
        # Initiate a withdraw
        await api_instance.post_withdraw(withdraw_request)
    except Exception as e:
        print("Exception when calling TradeApi->post_withdraw: %s\n" % e)
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

# **put_adjust_isolated_margin**
> put_adjust_isolated_margin(adjust_isolated_margin_request)

Adjust margin for an isolated position for a symbol

Adjust margin for an isolated position for a symbol

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.adjust_isolated_margin_request import AdjustIsolatedMarginRequest
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
    api_instance = openapi_client.TradeApi(api_client)
    adjust_isolated_margin_request = openapi_client.AdjustIsolatedMarginRequest() # AdjustIsolatedMarginRequest | 

    try:
        # Adjust margin for an isolated position for a symbol
        await api_instance.put_adjust_isolated_margin(adjust_isolated_margin_request)
    except Exception as e:
        print("Exception when calling TradeApi->put_adjust_isolated_margin: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **adjust_isolated_margin_request** | [**AdjustIsolatedMarginRequest**](AdjustIsolatedMarginRequest.md)|  | 

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
**202** | Adjust margin request sent successfully. |  -  |
**400** | Bad Request. The request was invalid or malformed. |  -  |
**401** | Unauthorized. Authentication is required or invalid. Signature is invalid |  -  |
**404** | Not Found. The requested account was not found. |  -  |
**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_authorize_account**
> put_authorize_account(account_authorization_request)

Authorizes an account

Authorizes an account to trade, perform liquidations and more, on behalf of another account

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.account_authorization_request import AccountAuthorizationRequest
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
    api_instance = openapi_client.TradeApi(api_client)
    account_authorization_request = openapi_client.AccountAuthorizationRequest() # AccountAuthorizationRequest | 

    try:
        # Authorizes an account
        await api_instance.put_authorize_account(account_authorization_request)
    except Exception as e:
        print("Exception when calling TradeApi->put_authorize_account: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_authorization_request** | [**AccountAuthorizationRequest**](AccountAuthorizationRequest.md)|  | 

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
**202** | Authorize Account request submitted successfully |  -  |
**400** | Bad Request. The request was invalid or malformed. |  -  |
**401** | Unauthorized. Authentication is required or invalid. Signature is invalid |  -  |
**500** | Internal Server Error. An unexpected error occurred on the server. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_deauthorize_account**
> put_deauthorize_account(account_authorization_request)

Deauthorizes an account

Deauthorizes an account to trade, perform liquidations and more, on behalf of another account

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.account_authorization_request import AccountAuthorizationRequest
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
    api_instance = openapi_client.TradeApi(api_client)
    account_authorization_request = openapi_client.AccountAuthorizationRequest() # AccountAuthorizationRequest | 

    try:
        # Deauthorizes an account
        await api_instance.put_deauthorize_account(account_authorization_request)
    except Exception as e:
        print("Exception when calling TradeApi->put_deauthorize_account: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_authorization_request** | [**AccountAuthorizationRequest**](AccountAuthorizationRequest.md)|  | 

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
**202** | Deauthorize Account request submitted successfully |  -  |
**400** | Bad Request. The request was invalid or malformed. |  -  |
**401** | Unauthorized. Authentication is required or invalid. Signature is invalid |  -  |
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
    api_instance = openapi_client.TradeApi(api_client)
    account_position_leverage_update_request = openapi_client.AccountPositionLeverageUpdateRequest() # AccountPositionLeverageUpdateRequest | 

    try:
        # Updates leverage for positions
        await api_instance.put_leverage_update(account_position_leverage_update_request)
    except Exception as e:
        print("Exception when calling TradeApi->put_leverage_update: %s\n" % e)
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

