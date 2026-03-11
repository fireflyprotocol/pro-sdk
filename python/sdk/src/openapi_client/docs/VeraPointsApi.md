# openapi_client.VeraPointsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**claim_swap**](VeraPointsApi.md#claim_swap) | **POST** /v1/vera/swap/claim | /vera/swap/claim
[**claim_vault**](VeraPointsApi.md#claim_vault) | **POST** /v1/vera/vault/claim | /vera/vault/claim
[**get_balance**](VeraPointsApi.md#get_balance) | **GET** /v1/vera/balance | Get user&#39;s points, tier, and rank
[**get_leaderboard**](VeraPointsApi.md#get_leaderboard) | **GET** /v1/vera/leaderboard | Top users by lifetime points
[**record_session**](VeraPointsApi.md#record_session) | **POST** /v1/vera/session | /vera/session


# **claim_swap**
> SwapClaimResponse claim_swap(swap_claim_request)

/vera/swap/claim

Claim a swap transaction for Vera Points attribution.


### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.swap_claim_request import SwapClaimRequest
from openapi_client.models.swap_claim_response import SwapClaimResponse
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
    api_instance = openapi_client.VeraPointsApi(api_client)
    swap_claim_request = openapi_client.SwapClaimRequest() # SwapClaimRequest | 

    try:
        # /vera/swap/claim
        api_response = await api_instance.claim_swap(swap_claim_request)
        print("The response of VeraPointsApi->claim_swap:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling VeraPointsApi->claim_swap: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **swap_claim_request** | [**SwapClaimRequest**](SwapClaimRequest.md)|  | 

### Return type

[**SwapClaimResponse**](SwapClaimResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Swap claim recorded successfully |  -  |
**400** | Invalid or missing transaction hash |  -  |
**401** | Missing or invalid authentication token |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **claim_vault**
> VaultClaimResponse claim_vault(vault_claim_request)

/vera/vault/claim

Claim a vault deposit transaction for Vera Points attribution.


### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.vault_claim_request import VaultClaimRequest
from openapi_client.models.vault_claim_response import VaultClaimResponse
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
    api_instance = openapi_client.VeraPointsApi(api_client)
    vault_claim_request = openapi_client.VaultClaimRequest() # VaultClaimRequest | 

    try:
        # /vera/vault/claim
        api_response = await api_instance.claim_vault(vault_claim_request)
        print("The response of VeraPointsApi->claim_vault:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling VeraPointsApi->claim_vault: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **vault_claim_request** | [**VaultClaimRequest**](VaultClaimRequest.md)|  | 

### Return type

[**VaultClaimResponse**](VaultClaimResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Vault claim recorded successfully |  -  |
**400** | Invalid or missing transaction hash |  -  |
**401** | Missing or invalid authentication token |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_balance**
> BalanceResponse get_balance(user_address)

Get user's points, tier, and rank

Returns a user's lifetime Vera Points, current tier, rank, and next tier threshold.
Public endpoint; user_address is passed as query parameter.


### Example


```python
import openapi_client
from openapi_client.models.balance_response import BalanceResponse
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
    api_instance = openapi_client.VeraPointsApi(api_client)
    user_address = 'user_address_example' # str | Wallet address to look up balance for.

    try:
        # Get user's points, tier, and rank
        api_response = await api_instance.get_balance(user_address)
        print("The response of VeraPointsApi->get_balance:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling VeraPointsApi->get_balance: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_address** | **str**| Wallet address to look up balance for. | 

### Return type

[**BalanceResponse**](BalanceResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | User&#39;s balance and tier info |  -  |
**400** | Missing or invalid userAddress |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_leaderboard**
> LeaderboardResponse get_leaderboard(limit=limit, offset=offset)

Top users by lifetime points

Paginated leaderboard ranked by lifetime Vera Points.

### Example


```python
import openapi_client
from openapi_client.models.leaderboard_response import LeaderboardResponse
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
    api_instance = openapi_client.VeraPointsApi(api_client)
    limit = 20 # int | Number of entries to return. (optional) (default to 20)
    offset = 0 # int | Number of entries to skip. (optional) (default to 0)

    try:
        # Top users by lifetime points
        api_response = await api_instance.get_leaderboard(limit=limit, offset=offset)
        print("The response of VeraPointsApi->get_leaderboard:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling VeraPointsApi->get_leaderboard: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **int**| Number of entries to return. | [optional] [default to 20]
 **offset** | **int**| Number of entries to skip. | [optional] [default to 0]

### Return type

[**LeaderboardResponse**](LeaderboardResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Leaderboard entries |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **record_session**
> SessionResponse record_session()

/vera/session

Record a daily app session for vault points eligibility.
user_address is extracted from JWT; session_date is set server-side to the current UTC date.
Idempotent — one session per (user, date) pair; multiple calls on the same day are no-ops.
This is a required daily gate for vault balance points.


### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.session_response import SessionResponse
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
    api_instance = openapi_client.VeraPointsApi(api_client)

    try:
        # /vera/session
        api_response = await api_instance.record_session()
        print("The response of VeraPointsApi->record_session:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling VeraPointsApi->record_session: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**SessionResponse**](SessionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Session recorded successfully |  -  |
**401** | Missing or invalid authentication token |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

