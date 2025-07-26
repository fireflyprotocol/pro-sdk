# openapi_client.RewardsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_affiliate_interval_overview**](RewardsApi.md#get_affiliate_interval_overview) | **GET** /v1/rewards/affiliate/intervalOverview | /rewards/affiliate/intervalOverview
[**get_affiliate_leader_dashboard**](RewardsApi.md#get_affiliate_leader_dashboard) | **GET** /v1/rewards/affiliate/leaderDashboard | /rewards/affiliate/leaderDashboard
[**get_affiliate_metadata**](RewardsApi.md#get_affiliate_metadata) | **GET** /v1/rewards/affiliate | /rewards/affiliate
[**get_affiliate_overview**](RewardsApi.md#get_affiliate_overview) | **GET** /v1/rewards/affiliate/overview | /rewards/affiliate/overview
[**get_affiliate_summary**](RewardsApi.md#get_affiliate_summary) | **GET** /v1/rewards/affiliate/summary | /rewards/affiliate/summary
[**get_campaign_rewards**](RewardsApi.md#get_campaign_rewards) | **GET** /v1/rewards/campaign | /rewards/campaign
[**get_rewards**](RewardsApi.md#get_rewards) | **GET** /v1/rewards | /rewards
[**get_rewards_campaign_metadata**](RewardsApi.md#get_rewards_campaign_metadata) | **GET** /v1/rewards/metadata/campaign | /rewards/metadata/campaign
[**get_rewards_epoch_config_metadata**](RewardsApi.md#get_rewards_epoch_config_metadata) | **GET** /v1/rewards/metadata/epoch/configs | /rewards/metadata/epoch/configs
[**get_rewards_epoch_metadata**](RewardsApi.md#get_rewards_epoch_metadata) | **GET** /v1/rewards/metadata/epoch | /rewards/metadata/epoch
[**get_rewards_interval_metadata**](RewardsApi.md#get_rewards_interval_metadata) | **GET** /v1/rewards/metadata/interval | /rewards/metadata/interval
[**get_rewards_summary**](RewardsApi.md#get_rewards_summary) | **GET** /v1/rewards/summary | /rewards/summary
[**onboard_affiliate**](RewardsApi.md#onboard_affiliate) | **POST** /v1/rewards/affiliate/onboard | /rewards/affiliate/onboard
[**onboard_referee**](RewardsApi.md#onboard_referee) | **POST** /v1/rewards/affiliate/onboard/referee | /rewards/affiliate/onboard/referee
[**update_affiliate_fee_config**](RewardsApi.md#update_affiliate_fee_config) | **POST** /v1/rewards/affiliate/feeConfig | /rewards/affiliate/feeConfig


# **get_affiliate_interval_overview**
> GetAffiliateIntervalOverview200Response get_affiliate_interval_overview(user_address, page=page, limit=limit)

/rewards/affiliate/intervalOverview

Returns detailed earnings breakdown for an affiliate by interval, ordered by interval number in descending order.

### Example


```python
import openapi_client
from openapi_client.models.get_affiliate_interval_overview200_response import GetAffiliateIntervalOverview200Response
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
    user_address = '0x1234567890abcdef' # str | The address of the user to get interval overview for
    page = 1 # int | The page number to retrieve in a paginated response (optional) (default to 1)
    limit = 500 # int | The page size for pagination (optional) (default to 500)

    try:
        # /rewards/affiliate/intervalOverview
        api_response = await api_instance.get_affiliate_interval_overview(user_address, page=page, limit=limit)
        print("The response of RewardsApi->get_affiliate_interval_overview:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->get_affiliate_interval_overview: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_address** | **str**| The address of the user to get interval overview for | 
 **page** | **int**| The page number to retrieve in a paginated response | [optional] [default to 1]
 **limit** | **int**| The page size for pagination | [optional] [default to 500]

### Return type

[**GetAffiliateIntervalOverview200Response**](GetAffiliateIntervalOverview200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response array of affiliate interval overview objects |  -  |
**401** | unauthorized access |  -  |
**404** | address not found |  -  |
**400** | request missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_affiliate_leader_dashboard**
> GetAffiliateLeaderDashboard200Response get_affiliate_leader_dashboard(sort_by=sort_by, sort_order=sort_order, page=page, limit=limit, search=search)

/rewards/affiliate/leaderDashboard

Returns rankings and earnings for affiliates, sorted by the specified category.

### Example


```python
import openapi_client
from openapi_client.models.get_affiliate_leader_dashboard200_response import GetAffiliateLeaderDashboard200Response
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
    sort_by = perpsRank # str | The category to sort rankings by (optional) (default to perpsRank)
    sort_order = 'desc' # str | The order to sort rankings by (optional)
    page = 1 # int | The page number to retrieve in a paginated response (optional) (default to 1)
    limit = 500 # int | The page size for pagination (optional) (default to 500)
    search = 'John' # str | The name/address of the user to filter by (optional)

    try:
        # /rewards/affiliate/leaderDashboard
        api_response = await api_instance.get_affiliate_leader_dashboard(sort_by=sort_by, sort_order=sort_order, page=page, limit=limit, search=search)
        print("The response of RewardsApi->get_affiliate_leader_dashboard:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->get_affiliate_leader_dashboard: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort_by** | **str**| The category to sort rankings by | [optional] [default to perpsRank]
 **sort_order** | **str**| The order to sort rankings by | [optional] 
 **page** | **int**| The page number to retrieve in a paginated response | [optional] [default to 1]
 **limit** | **int**| The page size for pagination | [optional] [default to 500]
 **search** | **str**| The name/address of the user to filter by | [optional] 

### Return type

[**GetAffiliateLeaderDashboard200Response**](GetAffiliateLeaderDashboard200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response array of affiliate ranking and earnings objects |  -  |
**401** | unauthorized access |  -  |
**404** | address not found |  -  |
**400** | request missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_affiliate_metadata**
> AffiliateMetadata get_affiliate_metadata(user_address)

/rewards/affiliate

Returns the affiliate metadata.

### Example


```python
import openapi_client
from openapi_client.models.affiliate_metadata import AffiliateMetadata
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
    user_address = '0x1234567890abcdef' # str | Specify wallet address.

    try:
        # /rewards/affiliate
        api_response = await api_instance.get_affiliate_metadata(user_address)
        print("The response of RewardsApi->get_affiliate_metadata:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->get_affiliate_metadata: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_address** | **str**| Specify wallet address. | 

### Return type

[**AffiliateMetadata**](AffiliateMetadata.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response |  -  |
**401** | unauthorized access |  -  |
**404** | address not found |  -  |
**400** | request missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_affiliate_overview**
> GetAffiliateOverview200Response get_affiliate_overview(user_address, page=page, limit=limit, sort_by=sort_by, sort_order=sort_order, search=search, min_earnings_e9=min_earnings_e9)

/rewards/affiliate/overview

Returns detailed earnings breakdown for an affiliate users earnings (including perps, spot LP, lending), referral earnings, and total earnings.

### Example


```python
import openapi_client
from openapi_client.models.get_affiliate_overview200_response import GetAffiliateOverview200Response
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
    user_address = '0x1234567890abcdef' # str | Specify wallet address.
    page = 1 # int | The page number to retrieve in a paginated response (optional) (default to 1)
    limit = 500 # int | The page size for pagination (optional) (default to 500)
    sort_by = totalEarnings # str | The category to sort earnings by (optional) (default to totalEarnings)
    sort_order = 'desc' # str | The order to sort earnings by (optional)
    search = 'John' # str | The name/address of the user to filter by (optional)
    min_earnings_e9 = '0' # str | The minimum earnings to filter by (optional) (default to '0')

    try:
        # /rewards/affiliate/overview
        api_response = await api_instance.get_affiliate_overview(user_address, page=page, limit=limit, sort_by=sort_by, sort_order=sort_order, search=search, min_earnings_e9=min_earnings_e9)
        print("The response of RewardsApi->get_affiliate_overview:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->get_affiliate_overview: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_address** | **str**| Specify wallet address. | 
 **page** | **int**| The page number to retrieve in a paginated response | [optional] [default to 1]
 **limit** | **int**| The page size for pagination | [optional] [default to 500]
 **sort_by** | **str**| The category to sort earnings by | [optional] [default to totalEarnings]
 **sort_order** | **str**| The order to sort earnings by | [optional] 
 **search** | **str**| The name/address of the user to filter by | [optional] 
 **min_earnings_e9** | **str**| The minimum earnings to filter by | [optional] [default to &#39;0&#39;]

### Return type

[**GetAffiliateOverview200Response**](GetAffiliateOverview200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response array of affiliate overview objects |  -  |
**401** | unauthorized access |  -  |
**404** | address not found |  -  |
**400** | request missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_affiliate_summary**
> AffiliateSummary get_affiliate_summary(user_address)

/rewards/affiliate/summary

Returns performance summary for an affiliate including total referrals, earnings, and rankings.

### Example


```python
import openapi_client
from openapi_client.models.affiliate_summary import AffiliateSummary
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
    user_address = '0x1234567890abcdef' # str | Specify wallet address.

    try:
        # /rewards/affiliate/summary
        api_response = await api_instance.get_affiliate_summary(user_address)
        print("The response of RewardsApi->get_affiliate_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->get_affiliate_summary: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_address** | **str**| Specify wallet address. | 

### Return type

[**AffiliateSummary**](AffiliateSummary.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response of affiliate summary |  -  |
**401** | unauthorized access |  -  |
**404** | address not found |  -  |
**400** | request missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_campaign_rewards**
> List[UserCampaignRewards] get_campaign_rewards(campaign_name, user_address, epoch_number=epoch_number)

/rewards/campaign

Returns the rewards earned by users for a specific campaign.

### Example


```python
import openapi_client
from openapi_client.models.user_campaign_rewards import UserCampaignRewards
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
    campaign_name = 'TRADE_AND_EARN' # str | Specify the campaign name
    user_address = '0x1234567890abcdef' # str | Specify wallet address.
    epoch_number = 7 # int | Optionally specify epoch number. (optional)

    try:
        # /rewards/campaign
        api_response = await api_instance.get_campaign_rewards(campaign_name, user_address, epoch_number=epoch_number)
        print("The response of RewardsApi->get_campaign_rewards:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->get_campaign_rewards: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **campaign_name** | **str**| Specify the campaign name | 
 **user_address** | **str**| Specify wallet address. | 
 **epoch_number** | **int**| Optionally specify epoch number. | [optional] 

### Return type

[**List[UserCampaignRewards]**](UserCampaignRewards.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response |  -  |
**400** | request missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rewards**
> List[IntervalRewards] get_rewards(interval_number=interval_number)

/rewards

Returns the rewards earned by users for the intervals.

### Example

* Bearer (JWT) Authentication (bearerAuth):

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
    api_instance = openapi_client.RewardsApi(api_client)
    interval_number = 3 # int | Optionally specify interval number. (optional)

    try:
        # /rewards
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

[bearerAuth](../README.md#bearerAuth)

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

/rewards/metadata/campaign

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
    campaign_name = 'TRADE_AND_EARN' # str | Specify the campaign name (optional)
    status = ACTIVE # str | Optionally specify the status of the campaigns. (optional) (default to ACTIVE)

    try:
        # /rewards/metadata/campaign
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
> EpochConfigsResponse get_rewards_epoch_config_metadata()

/rewards/metadata/epoch/configs

Returns the latest epoch configs for the campaigns.

### Example


```python
import openapi_client
from openapi_client.models.epoch_configs_response import EpochConfigsResponse
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
        # /rewards/metadata/epoch/configs
        api_response = await api_instance.get_rewards_epoch_config_metadata()
        print("The response of RewardsApi->get_rewards_epoch_config_metadata:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->get_rewards_epoch_config_metadata: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**EpochConfigsResponse**](EpochConfigsResponse.md)

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

/rewards/metadata/epoch

Returns the latest or next epoch epoch for campaign.

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
    campaign_name = 'TRADE_AND_EARN' # str | Specify the campaign name (optional)
    epoch = 'epoch_example' # str | Specify the string \"next\" or \"latest\". (optional)

    try:
        # /rewards/metadata/epoch
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

/rewards/metadata/interval

Returns the interval metadata for provided parameters.

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
        # /rewards/metadata/interval
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

/rewards/summary

Returns the all time rewards earned by users.

### Example

* Bearer (JWT) Authentication (bearerAuth):

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
    api_instance = openapi_client.RewardsApi(api_client)

    try:
        # /rewards/summary
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

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **onboard_affiliate**
> AffiliateOnboardResponse onboard_affiliate(onboard_affiliate_request)

/rewards/affiliate/onboard

Submit an application to become an affiliate.

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.affiliate_onboard_response import AffiliateOnboardResponse
from openapi_client.models.onboard_affiliate_request import OnboardAffiliateRequest
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
    api_instance = openapi_client.RewardsApi(api_client)
    onboard_affiliate_request = openapi_client.OnboardAffiliateRequest() # OnboardAffiliateRequest | 

    try:
        # /rewards/affiliate/onboard
        api_response = await api_instance.onboard_affiliate(onboard_affiliate_request)
        print("The response of RewardsApi->onboard_affiliate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->onboard_affiliate: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **onboard_affiliate_request** | [**OnboardAffiliateRequest**](OnboardAffiliateRequest.md)|  | 

### Return type

[**AffiliateOnboardResponse**](AffiliateOnboardResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Application submitted successfully |  -  |
**401** | unauthorized access |  -  |
**404** | address not found |  -  |
**400** | request missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **onboard_referee**
> RefereeOnboardResponse onboard_referee(onboard_referee_request)

/rewards/affiliate/onboard/referee

Onboard a referee with a referral code.

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.onboard_referee_request import OnboardRefereeRequest
from openapi_client.models.referee_onboard_response import RefereeOnboardResponse
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
    api_instance = openapi_client.RewardsApi(api_client)
    onboard_referee_request = openapi_client.OnboardRefereeRequest() # OnboardRefereeRequest | 

    try:
        # /rewards/affiliate/onboard/referee
        api_response = await api_instance.onboard_referee(onboard_referee_request)
        print("The response of RewardsApi->onboard_referee:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->onboard_referee: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **onboard_referee_request** | [**OnboardRefereeRequest**](OnboardRefereeRequest.md)|  | 

### Return type

[**RefereeOnboardResponse**](RefereeOnboardResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Referee onboarding response |  -  |
**401** | unauthorized access |  -  |
**404** | parent referral code not found |  -  |
**400** | referral code is required |  -  |
**412** | parent affiliate is not active |  -  |
**409** | referee already exists |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_affiliate_fee_config**
> AffiliateMetadata update_affiliate_fee_config(update_affiliate_fee_config_request)

/rewards/affiliate/feeConfig

Update the fee config for an affiliate.

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import openapi_client
from openapi_client.models.affiliate_metadata import AffiliateMetadata
from openapi_client.models.update_affiliate_fee_config_request import UpdateAffiliateFeeConfigRequest
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
    api_instance = openapi_client.RewardsApi(api_client)
    update_affiliate_fee_config_request = openapi_client.UpdateAffiliateFeeConfigRequest() # UpdateAffiliateFeeConfigRequest | 

    try:
        # /rewards/affiliate/feeConfig
        api_response = await api_instance.update_affiliate_fee_config(update_affiliate_fee_config_request)
        print("The response of RewardsApi->update_affiliate_fee_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RewardsApi->update_affiliate_fee_config: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **update_affiliate_fee_config_request** | [**UpdateAffiliateFeeConfigRequest**](UpdateAffiliateFeeConfigRequest.md)|  | 

### Return type

[**AffiliateMetadata**](AffiliateMetadata.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Cashback request processed successfully |  -  |
**401** | unauthorized access |  -  |
**404** | address not found |  -  |
**400** | request missing required parameters |  -  |
**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

