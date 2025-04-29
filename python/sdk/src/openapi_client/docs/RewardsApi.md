# openapi_client.RewardsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_rewards_campaign_get**](RewardsApi.md#v1_rewards_campaign_get) | **GET** /v1/rewards/campaign | Get rewards information for a specific campaign
[**v1_rewards_get**](RewardsApi.md#v1_rewards_get) | **GET** /v1/rewards | Get rewards information for the intervals
[**v1_rewards_metadata_campaign_get**](RewardsApi.md#v1_rewards_metadata_campaign_get) | **GET** /v1/rewards/metadata/campaign | Get rewards metadata for the campaigns
[**v1_rewards_metadata_epoch_configs_get**](RewardsApi.md#v1_rewards_metadata_epoch_configs_get) | **GET** /v1/rewards/metadata/epoch/configs | Gets the latest epoch configs for the campaigns
[**v1_rewards_metadata_epoch_get**](RewardsApi.md#v1_rewards_metadata_epoch_get) | **GET** /v1/rewards/metadata/epoch | Gets the latest or next epoch for campaign.
[**v1_rewards_metadata_interval_get**](RewardsApi.md#v1_rewards_metadata_interval_get) | **GET** /v1/rewards/metadata/interval | Gets the interval metadata for provided parameters
[**v1_rewards_summary_get**](RewardsApi.md#v1_rewards_summary_get) | **GET** /v1/rewards/summary | Get rewards information for all time rewards earned


# **v1_rewards_campaign_get**
> List[CampaignRewards] v1_rewards_campaign_get(campaign_name, epoch_number=epoch_number)

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
        api_response = await api_instance.v1_rewards_campaign_get(campaign_name, epoch_number=epoch_number)
        print("The response of RewardsApi->v1_rewards_campaign_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->v1_rewards_campaign_get: %s\n" % e)
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

# **v1_rewards_get**
> List[IntervalRewards] v1_rewards_get(interval_number=interval_number)

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
        api_response = await api_instance.v1_rewards_get(interval_number=interval_number)
        print("The response of RewardsApi->v1_rewards_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->v1_rewards_get: %s\n" % e)
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

# **v1_rewards_metadata_campaign_get**
> List[CampaignMetadata] v1_rewards_metadata_campaign_get(campaign_name=campaign_name, status=status)

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
        api_response = await api_instance.v1_rewards_metadata_campaign_get(campaign_name=campaign_name, status=status)
        print("The response of RewardsApi->v1_rewards_metadata_campaign_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->v1_rewards_metadata_campaign_get: %s\n" % e)
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

# **v1_rewards_metadata_epoch_configs_get**
> List[EpochConfigs] v1_rewards_metadata_epoch_configs_get()

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
        api_response = await api_instance.v1_rewards_metadata_epoch_configs_get()
        print("The response of RewardsApi->v1_rewards_metadata_epoch_configs_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->v1_rewards_metadata_epoch_configs_get: %s\n" % e)
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

# **v1_rewards_metadata_epoch_get**
> List[EpochMetadata] v1_rewards_metadata_epoch_get(campaign_name=campaign_name, epoch=epoch)

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
        api_response = await api_instance.v1_rewards_metadata_epoch_get(campaign_name=campaign_name, epoch=epoch)
        print("The response of RewardsApi->v1_rewards_metadata_epoch_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->v1_rewards_metadata_epoch_get: %s\n" % e)
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

# **v1_rewards_metadata_interval_get**
> List[IntervalMetadata] v1_rewards_metadata_interval_get(interval=interval)

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
    interval = openapi_client.V1RewardsMetadataIntervalGetIntervalParameter() # V1RewardsMetadataIntervalGetIntervalParameter | Either specify an interval number or the string \"next\" or \"latest\". (optional)

    try:
        # Gets the interval metadata for provided parameters
        api_response = await api_instance.v1_rewards_metadata_interval_get(interval=interval)
        print("The response of RewardsApi->v1_rewards_metadata_interval_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->v1_rewards_metadata_interval_get: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **interval** | [**V1RewardsMetadataIntervalGetIntervalParameter**](.md)| Either specify an interval number or the string \&quot;next\&quot; or \&quot;latest\&quot;. | [optional] 

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

# **v1_rewards_summary_get**
> List[RewardsSummary] v1_rewards_summary_get()

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
        api_response = await api_instance.v1_rewards_summary_get()
        print("The response of RewardsApi->v1_rewards_summary_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->v1_rewards_summary_get: %s\n" % e)
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

