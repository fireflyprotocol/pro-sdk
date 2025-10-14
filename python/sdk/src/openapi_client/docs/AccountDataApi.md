# openapi_client.AccountDataApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_details**](AccountDataApi.md#get_account_details) | **GET** /api/v1/account | /account
[**get_account_funding_rate_history**](AccountDataApi.md#get_account_funding_rate_history) | **GET** /api/v1/account/fundingRateHistory | /account/fundingRateHistory
[**get_account_preferences**](AccountDataApi.md#get_account_preferences) | **GET** /api/v1/account/preferences | /account/preferences
[**get_account_trades**](AccountDataApi.md#get_account_trades) | **GET** /api/v1/account/trades | /account/trades
[**get_account_transaction_history**](AccountDataApi.md#get_account_transaction_history) | **GET** /api/v1/account/transactions | /account/transactions
[**patch_account_group_id**](AccountDataApi.md#patch_account_group_id) | **PATCH** /api/v1/account/groupId | Set the group ID for an account.
[**put_account_preferences**](AccountDataApi.md#put_account_preferences) | **PUT** /api/v1/account/preferences | /account/preferences
[**sponsor_tx**](AccountDataApi.md#sponsor_tx) | **POST** /api/v1/account/sponsorTx | /account/sponsorTx


# **get_account_details**
> Account get_account_details(account_address=account_address)

/account

Retrieves the user's account details.

### Example


```python
import openapi_client
from openapi_client.models.account import Account
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
    api_instance = openapi_client.AccountDataApi(api_client)
    account_address = 'account_address_example' # str | Account address to fetch account details by. (optional)

    try:
        # /account
        api_response = await api_instance.get_account_details(account_address=account_address)
        print("The response of AccountDataApi->get_account_details:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AccountDataApi->get_account_details: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_address** | **str**| Account address to fetch account details by. | [optional] 

### Return type

[**Account**](Account.md)

### Authorization

No authorization required

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
> AccountFundingRateHistory get_account_funding_rate_history(account_address=account_address, limit=limit, page=page, start_time_at_millis=start_time_at_millis, end_time_at_millis=end_time_at_millis)

/account/fundingRateHistory

Retrieves the funding rate history for a specific account.

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.account_funding_rate_history import AccountFundingRateHistory
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
    api_instance = openapi_client.AccountDataApi(api_client)
    account_address = 'account_address_example' # str | Account address to filter funding rate history by. (optional)
    limit = 500 # int | Default 500; max 1000. (optional) (default to 500)
    page = 1 # int | The page number to retrieve in a paginated response. (optional) (default to 1)
    start_time_at_millis = 56 # int | Start time in milliseconds. Defaults to 7 days ago if not specified. (optional)
    end_time_at_millis = 56 # int | End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 90 days apart. (optional)

    try:
        # /account/fundingRateHistory
        api_response = await api_instance.get_account_funding_rate_history(account_address=account_address, limit=limit, page=page, start_time_at_millis=start_time_at_millis, end_time_at_millis=end_time_at_millis)
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
 **start_time_at_millis** | **int**| Start time in milliseconds. Defaults to 7 days ago if not specified. | [optional] 
 **end_time_at_millis** | **int**| End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 90 days apart. | [optional] 

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

/account/preferences

Retrieves the user's account preferences.

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.account_preference import AccountPreference
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
    api_instance = openapi_client.AccountDataApi(api_client)

    try:
        # /account/preferences
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

/account/trades

Retrieves the user's trade history.

### Example

* Bearer (JWT) Authentication (bearerAuth):

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
    end_time_at_millis = 56 # int | End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 90 days apart. (optional)
    limit = 500 # int | Default 500; max 1000. (optional) (default to 500)
    trade_type = openapi_client.TradeType() # TradeType | Type of trade. By default returns all. UNSPECIFIED returns all. (optional)
    page = 56 # int | The page number to retrieve in a paginated response. (optional)

    try:
        # /account/trades
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
 **end_time_at_millis** | **int**| End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 90 days apart. | [optional] 
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

/account/transactions

Retrieves the user's transaction history (any change in balance).

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.transaction import Transaction
from openapi_client.models.transaction_type import TransactionType
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
    api_instance = openapi_client.AccountDataApi(api_client)
    types = [openapi_client.TransactionType()] # List[TransactionType] | Optional query parameter to filter transactions by type. (optional)
    asset_symbol = 'asset_symbol_example' # str | Optional query parameter to filter transactions by asset bank address. (optional)
    start_time_at_millis = 56 # int | Start time in milliseconds. Defaults to 7 days ago if not specified. (optional)
    end_time_at_millis = 56 # int | End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 90 days apart. (optional)
    limit = 500 # int | Default 500; max 1000. (optional) (default to 500)
    page = 56 # int | The page number to retrieve in a paginated response. (optional)

    try:
        # /account/transactions
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
 **end_time_at_millis** | **int**| End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 90 days apart. | [optional] 
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

# **patch_account_group_id**
> patch_account_group_id(account_group_id_patch)

Set the group ID for an account.

Sets or updates the group ID for a specific account.
Accounts belonging to the same group cannot trade against each other.
If the groupId is not set, the account will be removed from its group.
Only the first 6 characters of the groupID are guaranteed to be respected,
longer group IDs may be rejected.


### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.account_group_id_patch import AccountGroupIdPatch
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
    api_instance = openapi_client.AccountDataApi(api_client)
    account_group_id_patch = openapi_client.AccountGroupIdPatch() # AccountGroupIdPatch | Account group ID update.

    try:
        # Set the group ID for an account.
        await api_instance.patch_account_group_id(account_group_id_patch)
    except Exception as e:
        print("Exception when calling AccountDataApi->patch_account_group_id: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_group_id_patch** | [**AccountGroupIdPatch**](AccountGroupIdPatch.md)| Account group ID update. | 

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
**202** | Accepted and processing. |  -  |
**400** | request missing required parameters |  -  |
**401** | unauthorized access |  -  |
**404** | account not found |  -  |
**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_account_preferences**
> put_account_preferences(update_account_preference_request)

/account/preferences

Update user's account preferences. This will overwrite the preferences, so always send the full object.

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.update_account_preference_request import UpdateAccountPreferenceRequest
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
    api_instance = openapi_client.AccountDataApi(api_client)
    update_account_preference_request = openapi_client.UpdateAccountPreferenceRequest() # UpdateAccountPreferenceRequest | 

    try:
        # /account/preferences
        await api_instance.put_account_preferences(update_account_preference_request)
    except Exception as e:
        print("Exception when calling AccountDataApi->put_account_preferences: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **update_account_preference_request** | [**UpdateAccountPreferenceRequest**](UpdateAccountPreferenceRequest.md)|  | 

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
**204** | Success. |  -  |
**400** | request missing request body |  -  |
**401** | unauthorized access |  -  |
**404** | account not found |  -  |
**413** | request entity too large |  -  |
**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sponsor_tx**
> SponsorTxResponse sponsor_tx(sponsor_tx_request)

/account/sponsorTx

Sponsors a transaction if it's eligible for sponsorship based on allowlisted methods and kinds.

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.sponsor_tx_request import SponsorTxRequest
from openapi_client.models.sponsor_tx_response import SponsorTxResponse
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
    api_instance = openapi_client.AccountDataApi(api_client)
    sponsor_tx_request = openapi_client.SponsorTxRequest() # SponsorTxRequest | 

    try:
        # /account/sponsorTx
        api_response = await api_instance.sponsor_tx(sponsor_tx_request)
        print("The response of AccountDataApi->sponsor_tx:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AccountDataApi->sponsor_tx: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sponsor_tx_request** | [**SponsorTxRequest**](SponsorTxRequest.md)|  | 

### Return type

[**SponsorTxResponse**](SponsorTxResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Transaction successfully sponsored. |  -  |
**400** | Invalid request - transaction not eligible for sponsorship or missing required parameters |  -  |
**401** | Unauthorized access - missing or invalid authentication token |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

