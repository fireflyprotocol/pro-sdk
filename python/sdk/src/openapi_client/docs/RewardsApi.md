# openapi_client.RewardsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_campaign_rewards**](RewardsApi.md#get_campaign_rewards) | **GET** /v1/rewards/campaign | Get rewards information for a specific campaign
[**get_rewards**](RewardsApi.md#get_rewards) | **GET** /v1/rewards | Get rewards information for the intervals
[**get_rewards_campaign_metadata**](RewardsApi.md#get_rewards_campaign_metadata) | **GET** /v1/rewards/metadata/campaign | Get rewards metadata for the campaigns
[**get_rewards_epoch_config_metadata**](RewardsApi.md#get_rewards_epoch_config_metadata) | **GET** /v1/rewards/metadata/epoch/configs | Gets the latest epoch configs for the campaigns
[**get_rewards_epoch_metadata**](RewardsApi.md#get_rewards_epoch_metadata) | **GET** /v1/rewards/metadata/epoch | Gets the latest or next epoch for campaign.
[**get_rewards_interval_metadata**](RewardsApi.md#get_rewards_interval_metadata) | **GET** /v1/rewards/metadata/interval | Gets the interval metadata for provided parameters
[**get_rewards_summary**](RewardsApi.md#get_rewards_summary) | **GET** /v1/rewards/summary | Get rewards information for all time rewards earned


# **get_campaign_rewards**
> List[CampaignRewards] get_campaign_rewards(campaign_name, epoch_number=epoch_number)

Get rewards information for a specific campaign

Returns the rewards earned by users for a specific campaign

### Example


```python
import openapi_client
from openapi_client.models.campaign_rewards import CampaignRewards
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
    api_instance = openapi_client.RewardsApi(api_client)
    campaign_name = 'Trade and Earn' # str | Specify the campaign name
    epoch_number = 7 # int | Optionally specify epoch number. (optional)

    try:
        # Get rewards information for a specific campaign
        api_response = await api_instance.get_campaign_rewards(campaign_name, epoch_number=epoch_number)
        print("The response of RewardsApi->get_campaign_rewards:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->get_campaign_rewards: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **campaign_name** | **str**| Specify the campaign name | 
 **epoch_number** | **int**| Optionally specify epoch number. | [optional] 

### Return type

[**List[CampaignRewards]**](CampaignRewards.md)

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

# **get_rewards**
> List[IntervalRewards] get_rewards(interval_number=interval_number)

Get rewards information for the intervals

Returns the rewards earned by users for the intervals .

### Example


```python
import openapi_client
from openapi_client.models.interval_rewards import IntervalRewards
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
    api_instance = openapi_client.RewardsApi(api_client)
    interval_number = 3 # int | Optionally specify interval number. (optional)

    try:
        # Get rewards information for the intervals
        api_response = await api_instance.get_rewards(interval_number=interval_number)
        print("The response of RewardsApi->get_rewards:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->get_rewards: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **interval_number** | **int**| Optionally specify interval number. | [optional] 

### Return type

[**List[IntervalRewards]**](IntervalRewards.md)

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

# **get_rewards_campaign_metadata**
> List[CampaignMetadata] get_rewards_campaign_metadata(campaign_name=campaign_name, status=status)

Get rewards metadata for the campaigns

Returns the metadata for the rewards campaigns.

### Example


```python
import openapi_client
from openapi_client.models.campaign_metadata import CampaignMetadata
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
    api_instance = openapi_client.RewardsApi(api_client)
    campaign_name = 'Trade and Earn' # str | Specify the campaign name (optional)
    status = ACTIVE # str | Optionally specify the status of the campaigns. (optional) (default to ACTIVE)

    try:
        # Get rewards metadata for the campaigns
        api_response = await api_instance.get_rewards_campaign_metadata(campaign_name=campaign_name, status=status)
        print("The response of RewardsApi->get_rewards_campaign_metadata:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->get_rewards_campaign_metadata: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **campaign_name** | **str**| Specify the campaign name | [optional] 
 **status** | **str**| Optionally specify the status of the campaigns. | [optional] [default to ACTIVE]

### Return type

[**List[CampaignMetadata]**](CampaignMetadata.md)

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

# **get_rewards_epoch_config_metadata**
> List[EpochConfigs] get_rewards_epoch_config_metadata()

Gets the latest epoch configs for the campaigns

Returns the latest epoch configs for the campaigns

### Example


```python
import openapi_client
from openapi_client.models.epoch_configs import EpochConfigs
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
    api_instance = openapi_client.RewardsApi(api_client)

    try:
        # Gets the latest epoch configs for the campaigns
        api_response = await api_instance.get_rewards_epoch_config_metadata()
        print("The response of RewardsApi->get_rewards_epoch_config_metadata:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->get_rewards_epoch_config_metadata: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**List[EpochConfigs]**](EpochConfigs.md)

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

# **get_rewards_epoch_metadata**
> List[EpochMetadata] get_rewards_epoch_metadata(campaign_name=campaign_name, epoch=epoch)

Gets the latest or next epoch for campaign.

Returns the latest or next epocht epoch for campaign.

### Example


```python
import openapi_client
from openapi_client.models.epoch_metadata import EpochMetadata
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
    api_instance = openapi_client.RewardsApi(api_client)
    campaign_name = 'Trade and Earn' # str | Specify the campaign name (optional)
    epoch = 'epoch_example' # str | Specify the string \"next\" or \"latest\". (optional)

    try:
        # Gets the latest or next epoch for campaign.
        api_response = await api_instance.get_rewards_epoch_metadata(campaign_name=campaign_name, epoch=epoch)
        print("The response of RewardsApi->get_rewards_epoch_metadata:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->get_rewards_epoch_metadata: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **campaign_name** | **str**| Specify the campaign name | [optional] 
 **epoch** | **str**| Specify the string \&quot;next\&quot; or \&quot;latest\&quot;. | [optional] 

### Return type

[**List[EpochMetadata]**](EpochMetadata.md)

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

# **get_rewards_interval_metadata**
> List[IntervalMetadata] get_rewards_interval_metadata(interval=interval)

Gets the interval metadata for provided parameters

Returns the interval metadata for provided parameters

### Example


```python
import openapi_client
from openapi_client.models.interval_metadata import IntervalMetadata
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
    api_instance = openapi_client.RewardsApi(api_client)
    interval = 2 # int | Either specify an interval number or the string \"next\" or \"latest\". (optional)

    try:
        # Gets the interval metadata for provided parameters
        api_response = await api_instance.get_rewards_interval_metadata(interval=interval)
        print("The response of RewardsApi->get_rewards_interval_metadata:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->get_rewards_interval_metadata: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **interval** | **int**| Either specify an interval number or the string \&quot;next\&quot; or \&quot;latest\&quot;. | [optional] 

### Return type

[**List[IntervalMetadata]**](IntervalMetadata.md)

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

# **get_rewards_summary**
> List[RewardsSummary] get_rewards_summary()

Get rewards information for all time rewards earned

Returns the all time rewards earned by users.

### Example


```python
import openapi_client
from openapi_client.models.rewards_summary import RewardsSummary
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
    api_instance = openapi_client.RewardsApi(api_client)

    try:
        # Get rewards information for all time rewards earned
        api_response = await api_instance.get_rewards_summary()
        print("The response of RewardsApi->get_rewards_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->get_rewards_summary: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**List[RewardsSummary]**](RewardsSummary.md)

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

