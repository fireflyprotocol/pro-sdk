# RewardsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**getCampaignRewards**](#getcampaignrewards) | **GET** /v1/rewards/campaign | Get rewards information for a specific campaign|
|[**getRewards**](#getrewards) | **GET** /v1/rewards | Get rewards information for the intervals|
|[**getRewardsCampaignMetadata**](#getrewardscampaignmetadata) | **GET** /v1/rewards/metadata/campaign | Get rewards metadata for the campaigns|
|[**getRewardsEpochConfigMetadata**](#getrewardsepochconfigmetadata) | **GET** /v1/rewards/metadata/epoch/configs | Gets the latest epoch configs for the campaigns|
|[**getRewardsEpochMetadata**](#getrewardsepochmetadata) | **GET** /v1/rewards/metadata/epoch | Gets the latest or next epoch for campaign.|
|[**getRewardsIntervalMetadata**](#getrewardsintervalmetadata) | **GET** /v1/rewards/metadata/interval | Gets the interval metadata for provided parameters|
|[**getRewardsSummary**](#getrewardssummary) | **GET** /v1/rewards/summary | Get rewards information for all time rewards earned|

# **getCampaignRewards**
> Array<CampaignRewards> getCampaignRewards()

Returns the rewards earned by users for a specific campaign

### Example

```typescript
import {
    RewardsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

let campaignName: string; //Specify the campaign name (default to undefined)
let epochNumber: number; //Optionally specify epoch number. (optional) (default to undefined)

const { status, data } = await apiInstance.getCampaignRewards(
    campaignName,
    epochNumber
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **campaignName** | [**string**] | Specify the campaign name | defaults to undefined|
| **epochNumber** | [**number**] | Optionally specify epoch number. | (optional) defaults to undefined|


### Return type

**Array<CampaignRewards>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRewards**
> Array<IntervalRewards> getRewards()

Returns the rewards earned by users for the intervals .

### Example

```typescript
import {
    RewardsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

let intervalNumber: number; //Optionally specify interval number. (optional) (default to undefined)

const { status, data } = await apiInstance.getRewards(
    intervalNumber
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **intervalNumber** | [**number**] | Optionally specify interval number. | (optional) defaults to undefined|


### Return type

**Array<IntervalRewards>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRewardsCampaignMetadata**
> Array<CampaignMetadata> getRewardsCampaignMetadata()

Returns the metadata for the rewards campaigns.

### Example

```typescript
import {
    RewardsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

let campaignName: string; //Specify the campaign name (optional) (default to undefined)
let status: 'ACTIVE' | 'INACTIVE'; //Optionally specify the status of the campaigns. (optional) (default to 'ACTIVE')

const { status, data } = await apiInstance.getRewardsCampaignMetadata(
    campaignName,
    status
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **campaignName** | [**string**] | Specify the campaign name | (optional) defaults to undefined|
| **status** | [**&#39;ACTIVE&#39; | &#39;INACTIVE&#39;**]**Array<&#39;ACTIVE&#39; &#124; &#39;INACTIVE&#39;>** | Optionally specify the status of the campaigns. | (optional) defaults to 'ACTIVE'|


### Return type

**Array<CampaignMetadata>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRewardsEpochConfigMetadata**
> Array<EpochConfigs> getRewardsEpochConfigMetadata()

Returns the latest epoch configs for the campaigns

### Example

```typescript
import {
    RewardsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

const { status, data } = await apiInstance.getRewardsEpochConfigMetadata();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<EpochConfigs>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRewardsEpochMetadata**
> Array<EpochMetadata> getRewardsEpochMetadata()

Returns the latest or next epocht epoch for campaign.

### Example

```typescript
import {
    RewardsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

let campaignName: string; //Specify the campaign name (optional) (default to undefined)
let epoch: 'next' | 'latest'; //Specify the string \"next\" or \"latest\". (optional) (default to undefined)

const { status, data } = await apiInstance.getRewardsEpochMetadata(
    campaignName,
    epoch
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **campaignName** | [**string**] | Specify the campaign name | (optional) defaults to undefined|
| **epoch** | [**&#39;next&#39; | &#39;latest&#39;**]**Array<&#39;next&#39; &#124; &#39;latest&#39;>** | Specify the string \&quot;next\&quot; or \&quot;latest\&quot;. | (optional) defaults to undefined|


### Return type

**Array<EpochMetadata>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRewardsIntervalMetadata**
> Array<IntervalMetadata> getRewardsIntervalMetadata()

Returns the interval metadata for provided parameters

### Example

```typescript
import {
    RewardsApi,
    Configuration,
    GetRewardsIntervalMetadataIntervalParameter
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

let interval: GetRewardsIntervalMetadataIntervalParameter; //Either specify an interval number or the string \"next\" or \"latest\". (optional) (default to undefined)

const { status, data } = await apiInstance.getRewardsIntervalMetadata(
    interval
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **interval** | **GetRewardsIntervalMetadataIntervalParameter** | Either specify an interval number or the string \&quot;next\&quot; or \&quot;latest\&quot;. | (optional) defaults to undefined|


### Return type

**Array<IntervalMetadata>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRewardsSummary**
> Array<RewardsSummary> getRewardsSummary()

Returns the all time rewards earned by users.

### Example

```typescript
import {
    RewardsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

const { status, data } = await apiInstance.getRewardsSummary();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<RewardsSummary>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

