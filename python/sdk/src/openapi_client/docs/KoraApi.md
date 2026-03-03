# openapi_client.KoraApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_kora_campaign_metadata**](KoraApi.md#get_kora_campaign_metadata) | **GET** /v1/kora/metadata/campaign | Get Kora campaign metadata
[**get_kora_campaign_rewards**](KoraApi.md#get_kora_campaign_rewards) | **GET** /v1/kora/rewards/campaign | Get Kora campaign rewards
[**get_kora_epoch_config_metadata**](KoraApi.md#get_kora_epoch_config_metadata) | **GET** /v1/kora/metadata/epoch/configs | Get Kora epoch configuration
[**get_kora_epoch_metadata**](KoraApi.md#get_kora_epoch_metadata) | **GET** /v1/kora/metadata/epoch | Get Kora epoch metadata
[**get_kora_interval_metadata**](KoraApi.md#get_kora_interval_metadata) | **GET** /v1/kora/metadata/interval | Get Kora interval metadata
[**get_kora_leaderboard**](KoraApi.md#get_kora_leaderboard) | **GET** /v1/kora/leaderboard | Get Kora swap leaderboard
[**get_kora_rewards_summary**](KoraApi.md#get_kora_rewards_summary) | **GET** /v1/kora/rewards/summary | Get Kora all-time rewards summary
[**kora_health_check**](KoraApi.md#kora_health_check) | **GET** /v1/kora/health | Kora service health check


# **get_kora_campaign_metadata**
> List[KoraCampaignMetadata] get_kora_campaign_metadata(campaign_name=campaign_name, status=status)

Get Kora campaign metadata

Returns metadata for Kora rewards campaigns.

### Example


```python
import openapi_client
from openapi_client.models.kora_campaign_metadata import KoraCampaignMetadata
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
    api_instance = openapi_client.KoraApi(api_client)
    campaign_name = 'KORA_SWAPS' # str | Specify the campaign name (optional)
    status = ACTIVE # str | Filter by campaign status (optional) (default to ACTIVE)

    try:
        # Get Kora campaign metadata
        api_response = await api_instance.get_kora_campaign_metadata(campaign_name=campaign_name, status=status)
        print("The response of KoraApi->get_kora_campaign_metadata:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling KoraApi->get_kora_campaign_metadata: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **campaign_name** | **str**| Specify the campaign name | [optional] 
 **status** | **str**| Filter by campaign status | [optional] [default to ACTIVE]

### Return type

[**List[KoraCampaignMetadata]**](KoraCampaignMetadata.md)

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

# **get_kora_campaign_rewards**
> List[KoraCampaignRewards] get_kora_campaign_rewards(user_address, campaign_name=campaign_name, epoch_number=epoch_number)

Get Kora campaign rewards

Returns the rewards earned by users for Kora campaigns.

### Example


```python
import openapi_client
from openapi_client.models.kora_campaign_rewards import KoraCampaignRewards
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
    api_instance = openapi_client.KoraApi(api_client)
    user_address = '0x1234567890abcdef' # str | Specify wallet address
    campaign_name = KORA_SWAPS # str | Specify the campaign name (optional) (default to KORA_SWAPS)
    epoch_number = 7 # int | Optionally specify epoch number (optional)

    try:
        # Get Kora campaign rewards
        api_response = await api_instance.get_kora_campaign_rewards(user_address, campaign_name=campaign_name, epoch_number=epoch_number)
        print("The response of KoraApi->get_kora_campaign_rewards:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling KoraApi->get_kora_campaign_rewards: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_address** | **str**| Specify wallet address | 
 **campaign_name** | **str**| Specify the campaign name | [optional] [default to KORA_SWAPS]
 **epoch_number** | **int**| Optionally specify epoch number | [optional] 

### Return type

[**List[KoraCampaignRewards]**](KoraCampaignRewards.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response |  -  |
**400** | Missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_kora_epoch_config_metadata**
> KoraEpochConfigResponse get_kora_epoch_config_metadata(interval_number=interval_number, campaign_name=campaign_name)

Get Kora epoch configuration

Returns epoch configuration including reward allocations for Kora.

### Example


```python
import openapi_client
from openapi_client.models.kora_epoch_config_response import KoraEpochConfigResponse
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
    api_instance = openapi_client.KoraApi(api_client)
    interval_number = 7 # int | Specify the interval number (optional)
    campaign_name = 'KORA_SWAPS' # str | Filter by campaign name (optional)

    try:
        # Get Kora epoch configuration
        api_response = await api_instance.get_kora_epoch_config_metadata(interval_number=interval_number, campaign_name=campaign_name)
        print("The response of KoraApi->get_kora_epoch_config_metadata:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling KoraApi->get_kora_epoch_config_metadata: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **interval_number** | **int**| Specify the interval number | [optional] 
 **campaign_name** | **str**| Filter by campaign name | [optional] 

### Return type

[**KoraEpochConfigResponse**](KoraEpochConfigResponse.md)

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

# **get_kora_epoch_metadata**
> List[KoraEpochMetadata] get_kora_epoch_metadata(campaign_name=campaign_name, epoch=epoch)

Get Kora epoch metadata

Returns the latest or next epoch for a Kora campaign.

### Example


```python
import openapi_client
from openapi_client.models.kora_epoch_metadata import KoraEpochMetadata
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
    api_instance = openapi_client.KoraApi(api_client)
    campaign_name = 'KORA_SWAPS' # str | Specify the campaign name (optional)
    epoch = 'epoch_example' # str | Specify \"next\" or \"latest\" (optional)

    try:
        # Get Kora epoch metadata
        api_response = await api_instance.get_kora_epoch_metadata(campaign_name=campaign_name, epoch=epoch)
        print("The response of KoraApi->get_kora_epoch_metadata:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling KoraApi->get_kora_epoch_metadata: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **campaign_name** | **str**| Specify the campaign name | [optional] 
 **epoch** | **str**| Specify \&quot;next\&quot; or \&quot;latest\&quot; | [optional] 

### Return type

[**List[KoraEpochMetadata]**](KoraEpochMetadata.md)

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

# **get_kora_interval_metadata**
> List[KoraIntervalMetadata] get_kora_interval_metadata(interval=interval, protocol=protocol)

Get Kora interval metadata

Returns interval metadata for Kora.

### Example


```python
import openapi_client
from openapi_client.models.kora_interval_metadata import KoraIntervalMetadata
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
    api_instance = openapi_client.KoraApi(api_client)
    interval = 'latest' # str | Interval number or \"next\"/\"latest\" (optional)
    protocol = kora # str | Filter by protocol (optional) (default to kora)

    try:
        # Get Kora interval metadata
        api_response = await api_instance.get_kora_interval_metadata(interval=interval, protocol=protocol)
        print("The response of KoraApi->get_kora_interval_metadata:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling KoraApi->get_kora_interval_metadata: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **interval** | **str**| Interval number or \&quot;next\&quot;/\&quot;latest\&quot; | [optional] 
 **protocol** | **str**| Filter by protocol | [optional] [default to kora]

### Return type

[**List[KoraIntervalMetadata]**](KoraIntervalMetadata.md)

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

# **get_kora_leaderboard**
> KoraLeaderboardResponse get_kora_leaderboard(epoch_id=epoch_id, sort_by=sort_by, sort_order=sort_order, page=page, limit=limit, search=search, min_volume_e6=min_volume_e6)

Get Kora swap leaderboard

Returns rankings and earnings for Kora swap participants, sorted by the specified category.

### Example


```python
import openapi_client
from openapi_client.models.kora_leaderboard_response import KoraLeaderboardResponse
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
    api_instance = openapi_client.KoraApi(api_client)
    epoch_id = 'epoch_id_example' # str | Specify epoch ID (defaults to current active epoch) (optional)
    sort_by = volumeRank # str | The category to sort rankings by (optional) (default to volumeRank)
    sort_order = desc # str | The order to sort rankings by (optional) (default to desc)
    page = 1 # int | Page number for pagination (optional) (default to 1)
    limit = 50 # int | Page size for pagination (optional) (default to 50)
    search = 'search_example' # str | Filter by user address (partial match supported) (optional)
    min_volume_e6 = '0' # str | Minimum trading volume filter (e6 format) (optional) (default to '0')

    try:
        # Get Kora swap leaderboard
        api_response = await api_instance.get_kora_leaderboard(epoch_id=epoch_id, sort_by=sort_by, sort_order=sort_order, page=page, limit=limit, search=search, min_volume_e6=min_volume_e6)
        print("The response of KoraApi->get_kora_leaderboard:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling KoraApi->get_kora_leaderboard: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **epoch_id** | **str**| Specify epoch ID (defaults to current active epoch) | [optional] 
 **sort_by** | **str**| The category to sort rankings by | [optional] [default to volumeRank]
 **sort_order** | **str**| The order to sort rankings by | [optional] [default to desc]
 **page** | **int**| Page number for pagination | [optional] [default to 1]
 **limit** | **int**| Page size for pagination | [optional] [default to 50]
 **search** | **str**| Filter by user address (partial match supported) | [optional] 
 **min_volume_e6** | **str**| Minimum trading volume filter (e6 format) | [optional] [default to &#39;0&#39;]

### Return type

[**KoraLeaderboardResponse**](KoraLeaderboardResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response |  -  |
**400** | Invalid request parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_kora_rewards_summary**
> List[KoraRewardsSummary] get_kora_rewards_summary()

Get Kora all-time rewards summary

Returns the all-time rewards earned by users including Kora CC and points.

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.kora_rewards_summary import KoraRewardsSummary
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
    api_instance = openapi_client.KoraApi(api_client)

    try:
        # Get Kora all-time rewards summary
        api_response = await api_instance.get_kora_rewards_summary()
        print("The response of KoraApi->get_kora_rewards_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling KoraApi->get_kora_rewards_summary: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**List[KoraRewardsSummary]**](KoraRewardsSummary.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **kora_health_check**
> KoraHealthCheck200Response kora_health_check()

Kora service health check

Returns the health status of the Kora rewards service.

### Example


```python
import openapi_client
from openapi_client.models.kora_health_check200_response import KoraHealthCheck200Response
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
    api_instance = openapi_client.KoraApi(api_client)

    try:
        # Kora service health check
        api_response = await api_instance.kora_health_check()
        print("The response of KoraApi->kora_health_check:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling KoraApi->kora_health_check: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**KoraHealthCheck200Response**](KoraHealthCheck200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Service is healthy |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

