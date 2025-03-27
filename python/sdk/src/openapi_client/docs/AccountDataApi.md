# openapi_client.AccountDataApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_details**](AccountDataApi.md#get_account_details) | **GET** /api/v1/account | Get user&#39;s account details.
[**get_account_funding_rate_history**](AccountDataApi.md#get_account_funding_rate_history) | **GET** /api/v1/account/fundingRateHistory | Get user&#39;s funding rate history
[**get_account_preferences**](AccountDataApi.md#get_account_preferences) | **GET** /api/v1/account/preferences | Get user&#39;s account preferences.
[**get_account_trades**](AccountDataApi.md#get_account_trades) | **GET** /api/v1/account/trades | Get user&#39;s trade history.
[**get_account_transaction_history**](AccountDataApi.md#get_account_transaction_history) | **GET** /api/v1/account/transactions | Get user&#39;s transaction history (any change in balance).


# **get_account_details**
> Account get_account_details()

Get user's account details.

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.account import Account
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
    api_instance = openapi_client.AccountDataApi(api_client)

    try:
        # Get user's account details.
        api_response = await api_instance.get_account_details()
        print("The response of AccountDataApi->get_account_details:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AccountDataApi->get_account_details: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**Account**](Account.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response with account details. |  -  |
**400** | request missing required parameters |  -  |
**401** | unauthorized access |  -  |
**404** | account not found |  -  |
**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_account_funding_rate_history**
> AccountFundingRateHistory get_account_funding_rate_history(account_address=account_address, limit=limit, page=page)

Get user's funding rate history

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.account_funding_rate_history import AccountFundingRateHistory
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
    api_instance = openapi_client.AccountDataApi(api_client)
    account_address = 'account_address_example' # str | Account address to filter funding rate history by. (optional)
    limit = 500 # int | Default 500; max 1000. (optional) (default to 500)
    page = 1 # int | The page number to retrieve in a paginated response. (optional) (default to 1)

    try:
        # Get user's funding rate history
        api_response = await api_instance.get_account_funding_rate_history(account_address=account_address, limit=limit, page=page)
        print("The response of AccountDataApi->get_account_funding_rate_history:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AccountDataApi->get_account_funding_rate_history: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_address** | **str**| Account address to filter funding rate history by. | [optional] 
 **limit** | **int**| Default 500; max 1000. | [optional] [default to 500]
 **page** | **int**| The page number to retrieve in a paginated response. | [optional] [default to 1]

### Return type

[**AccountFundingRateHistory**](AccountFundingRateHistory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response with funding rate history. |  -  |
**400** | request missing required parameters |  -  |
**401** | unauthorized access |  -  |
**404** | account not found |  -  |
**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_account_preferences**
> AccountPreference get_account_preferences()

Get user's account preferences.

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.account_preference import AccountPreference
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
    api_instance = openapi_client.AccountDataApi(api_client)

    try:
        # Get user's account preferences.
        api_response = await api_instance.get_account_preferences()
        print("The response of AccountDataApi->get_account_preferences:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AccountDataApi->get_account_preferences: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**AccountPreference**](AccountPreference.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response with account preferences. |  -  |
**400** | request missing required parameters |  -  |
**401** | unauthorized access |  -  |
**404** | account not found |  -  |
**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_account_trades**
> List[Trade] get_account_trades(symbol=symbol, start_time_at_millis=start_time_at_millis, end_time_at_millis=end_time_at_millis, limit=limit, trade_type=trade_type, page=page)

Get user's trade history.

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.trade import Trade
from openapi_client.models.trade_type import TradeType
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
    api_instance = openapi_client.AccountDataApi(api_client)
    symbol = 'symbol_example' # str | Market address to filter trades by. If not specified, returns trades for all markets. (optional)
    start_time_at_millis = 56 # int | Start time in milliseconds. Defaults to 7 days ago if not specified. (optional)
    end_time_at_millis = 56 # int | End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 7 days apart. (optional)
    limit = 500 # int | Default 500; max 1000. (optional) (default to 500)
    trade_type = openapi_client.TradeType() # TradeType | Type of trade. By default returns all. UNSPECIFIED returns all. (optional)
    page = 56 # int | The page number to retrieve in a paginated response. (optional)

    try:
        # Get user's trade history.
        api_response = await api_instance.get_account_trades(symbol=symbol, start_time_at_millis=start_time_at_millis, end_time_at_millis=end_time_at_millis, limit=limit, trade_type=trade_type, page=page)
        print("The response of AccountDataApi->get_account_trades:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AccountDataApi->get_account_trades: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **str**| Market address to filter trades by. If not specified, returns trades for all markets. | [optional] 
 **start_time_at_millis** | **int**| Start time in milliseconds. Defaults to 7 days ago if not specified. | [optional] 
 **end_time_at_millis** | **int**| End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 7 days apart. | [optional] 
 **limit** | **int**| Default 500; max 1000. | [optional] [default to 500]
 **trade_type** | [**TradeType**](.md)| Type of trade. By default returns all. UNSPECIFIED returns all. | [optional] 
 **page** | **int**| The page number to retrieve in a paginated response. | [optional] 

### Return type

[**List[Trade]**](Trade.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response with trade details. |  -  |
**400** | request missing required parameters |  -  |
**401** | unauthorized access |  -  |
**404** | account not found |  -  |
**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_account_transaction_history**
> List[Transaction] get_account_transaction_history(types=types, asset_symbol=asset_symbol, start_time_at_millis=start_time_at_millis, end_time_at_millis=end_time_at_millis, limit=limit, page=page)

Get user's transaction history (any change in balance).

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.transaction import Transaction
from openapi_client.models.transaction_type import TransactionType
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
    api_instance = openapi_client.AccountDataApi(api_client)
    types = [openapi_client.TransactionType()] # List[TransactionType] | Optional query parameter to filter transactions by type. (optional)
    asset_symbol = 'asset_symbol_example' # str | Optional query parameter to filter transactions by asset bank address. (optional)
    start_time_at_millis = 56 # int | Start time in milliseconds. Defaults to 7 days ago if not specified. (optional)
    end_time_at_millis = 56 # int | End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 7 days apart. (optional)
    limit = 500 # int | Default 500; max 1000. (optional) (default to 500)
    page = 56 # int | The page number to retrieve in a paginated response. (optional)

    try:
        # Get user's transaction history (any change in balance).
        api_response = await api_instance.get_account_transaction_history(types=types, asset_symbol=asset_symbol, start_time_at_millis=start_time_at_millis, end_time_at_millis=end_time_at_millis, limit=limit, page=page)
        print("The response of AccountDataApi->get_account_transaction_history:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AccountDataApi->get_account_transaction_history: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **types** | [**List[TransactionType]**](TransactionType.md)| Optional query parameter to filter transactions by type. | [optional] 
 **asset_symbol** | **str**| Optional query parameter to filter transactions by asset bank address. | [optional] 
 **start_time_at_millis** | **int**| Start time in milliseconds. Defaults to 7 days ago if not specified. | [optional] 
 **end_time_at_millis** | **int**| End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 7 days apart. | [optional] 
 **limit** | **int**| Default 500; max 1000. | [optional] [default to 500]
 **page** | **int**| The page number to retrieve in a paginated response. | [optional] 

### Return type

[**List[Transaction]**](Transaction.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response with transaction history details. |  -  |
**400** | request missing required parameters |  -  |
**401** | unauthorized access |  -  |
**404** | account not found |  -  |
**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

