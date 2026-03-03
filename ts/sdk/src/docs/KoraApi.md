# KoraApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**getKoraCampaignMetadata**](#getkoracampaignmetadata) | **GET** /v1/kora/metadata/campaign | Get Kora campaign metadata|
|[**getKoraCampaignRewards**](#getkoracampaignrewards) | **GET** /v1/kora/rewards/campaign | Get Kora campaign rewards|
|[**getKoraEpochConfigMetadata**](#getkoraepochconfigmetadata) | **GET** /v1/kora/metadata/epoch/configs | Get Kora epoch configuration|
|[**getKoraEpochMetadata**](#getkoraepochmetadata) | **GET** /v1/kora/metadata/epoch | Get Kora epoch metadata|
|[**getKoraIntervalMetadata**](#getkoraintervalmetadata) | **GET** /v1/kora/metadata/interval | Get Kora interval metadata|
|[**getKoraLeaderboard**](#getkoraleaderboard) | **GET** /v1/kora/leaderboard | Get Kora swap leaderboard|
|[**getKoraRewardsSummary**](#getkorarewardssummary) | **GET** /v1/kora/rewards/summary | Get Kora all-time rewards summary|
|[**koraHealthCheck**](#korahealthcheck) | **GET** /v1/kora/health | Kora service health check|

# **getKoraCampaignMetadata**
> Array<KoraCampaignMetadata> getKoraCampaignMetadata()

Returns metadata for Kora rewards campaigns.

### Example

```typescript
import {
    KoraApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new KoraApi(configuration);

let campaignName: string; //Specify the campaign name (optional) (default to undefined)
let status: 'ACTIVE' | 'INACTIVE'; //Filter by campaign status (optional) (default to 'ACTIVE')

const { status, data } = await apiInstance.getKoraCampaignMetadata(
    campaignName,
    status
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **campaignName** | [**string**] | Specify the campaign name | (optional) defaults to undefined|
| **status** | [**&#39;ACTIVE&#39; | &#39;INACTIVE&#39;**]**Array<&#39;ACTIVE&#39; &#124; &#39;INACTIVE&#39;>** | Filter by campaign status | (optional) defaults to 'ACTIVE'|


### Return type

**Array<KoraCampaignMetadata>**

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

# **getKoraCampaignRewards**
> Array<KoraCampaignRewards> getKoraCampaignRewards()

Returns the rewards earned by users for Kora campaigns.

### Example

```typescript
import {
    KoraApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new KoraApi(configuration);

let userAddress: string; //Specify wallet address (default to undefined)
let campaignName: 'KORA_SWAPS'; //Specify the campaign name (optional) (default to 'KORA_SWAPS')
let epochNumber: number; //Optionally specify epoch number (optional) (default to undefined)

const { status, data } = await apiInstance.getKoraCampaignRewards(
    userAddress,
    campaignName,
    epochNumber
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userAddress** | [**string**] | Specify wallet address | defaults to undefined|
| **campaignName** | [**&#39;KORA_SWAPS&#39;**]**Array<&#39;KORA_SWAPS&#39;>** | Specify the campaign name | (optional) defaults to 'KORA_SWAPS'|
| **epochNumber** | [**number**] | Optionally specify epoch number | (optional) defaults to undefined|


### Return type

**Array<KoraCampaignRewards>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |
|**400** | Missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getKoraEpochConfigMetadata**
> KoraEpochConfigResponse getKoraEpochConfigMetadata()

Returns epoch configuration including reward allocations for Kora.

### Example

```typescript
import {
    KoraApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new KoraApi(configuration);

let intervalNumber: number; //Specify the interval number (optional) (default to undefined)
let campaignName: string; //Filter by campaign name (optional) (default to undefined)

const { status, data } = await apiInstance.getKoraEpochConfigMetadata(
    intervalNumber,
    campaignName
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **intervalNumber** | [**number**] | Specify the interval number | (optional) defaults to undefined|
| **campaignName** | [**string**] | Filter by campaign name | (optional) defaults to undefined|


### Return type

**KoraEpochConfigResponse**

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

# **getKoraEpochMetadata**
> Array<KoraEpochMetadata> getKoraEpochMetadata()

Returns the latest or next epoch for a Kora campaign.

### Example

```typescript
import {
    KoraApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new KoraApi(configuration);

let campaignName: string; //Specify the campaign name (optional) (default to undefined)
let epoch: 'next' | 'latest'; //Specify \"next\" or \"latest\" (optional) (default to undefined)

const { status, data } = await apiInstance.getKoraEpochMetadata(
    campaignName,
    epoch
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **campaignName** | [**string**] | Specify the campaign name | (optional) defaults to undefined|
| **epoch** | [**&#39;next&#39; | &#39;latest&#39;**]**Array<&#39;next&#39; &#124; &#39;latest&#39;>** | Specify \&quot;next\&quot; or \&quot;latest\&quot; | (optional) defaults to undefined|


### Return type

**Array<KoraEpochMetadata>**

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

# **getKoraIntervalMetadata**
> Array<KoraIntervalMetadata> getKoraIntervalMetadata()

Returns interval metadata for Kora.

### Example

```typescript
import {
    KoraApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new KoraApi(configuration);

let interval: string; //Interval number or \"next\"/\"latest\" (optional) (default to undefined)
let protocol: 'bluefin' | 'kora'; //Filter by protocol (optional) (default to 'kora')

const { status, data } = await apiInstance.getKoraIntervalMetadata(
    interval,
    protocol
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **interval** | [**string**] | Interval number or \&quot;next\&quot;/\&quot;latest\&quot; | (optional) defaults to undefined|
| **protocol** | [**&#39;bluefin&#39; | &#39;kora&#39;**]**Array<&#39;bluefin&#39; &#124; &#39;kora&#39;>** | Filter by protocol | (optional) defaults to 'kora'|


### Return type

**Array<KoraIntervalMetadata>**

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

# **getKoraLeaderboard**
> KoraLeaderboardResponse getKoraLeaderboard()

Returns rankings and earnings for Kora swap participants, sorted by the specified category.

### Example

```typescript
import {
    KoraApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new KoraApi(configuration);

let epochId: string; //Specify epoch ID (defaults to current active epoch) (optional) (default to undefined)
let sortBy: 'volumeRank' | 'transactionRank' | 'totalEarnings'; //The category to sort rankings by (optional) (default to 'volumeRank')
let sortOrder: 'asc' | 'desc'; //The order to sort rankings by (optional) (default to 'desc')
let page: number; //Page number for pagination (optional) (default to 1)
let limit: number; //Page size for pagination (optional) (default to 50)
let search: string; //Filter by user address (partial match supported) (optional) (default to undefined)
let minVolumeE6: string; //Minimum trading volume filter (e6 format) (optional) (default to '0')

const { status, data } = await apiInstance.getKoraLeaderboard(
    epochId,
    sortBy,
    sortOrder,
    page,
    limit,
    search,
    minVolumeE6
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **epochId** | [**string**] | Specify epoch ID (defaults to current active epoch) | (optional) defaults to undefined|
| **sortBy** | [**&#39;volumeRank&#39; | &#39;transactionRank&#39; | &#39;totalEarnings&#39;**]**Array<&#39;volumeRank&#39; &#124; &#39;transactionRank&#39; &#124; &#39;totalEarnings&#39;>** | The category to sort rankings by | (optional) defaults to 'volumeRank'|
| **sortOrder** | [**&#39;asc&#39; | &#39;desc&#39;**]**Array<&#39;asc&#39; &#124; &#39;desc&#39;>** | The order to sort rankings by | (optional) defaults to 'desc'|
| **page** | [**number**] | Page number for pagination | (optional) defaults to 1|
| **limit** | [**number**] | Page size for pagination | (optional) defaults to 50|
| **search** | [**string**] | Filter by user address (partial match supported) | (optional) defaults to undefined|
| **minVolumeE6** | [**string**] | Minimum trading volume filter (e6 format) | (optional) defaults to '0'|


### Return type

**KoraLeaderboardResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |
|**400** | Invalid request parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getKoraRewardsSummary**
> Array<KoraRewardsSummary> getKoraRewardsSummary()

Returns the all-time rewards earned by users including Kora CC and points.

### Example

```typescript
import {
    KoraApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new KoraApi(configuration);

const { status, data } = await apiInstance.getKoraRewardsSummary();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<KoraRewardsSummary>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **koraHealthCheck**
> KoraHealthCheck200Response koraHealthCheck()

Returns the health status of the Kora rewards service.

### Example

```typescript
import {
    KoraApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new KoraApi(configuration);

const { status, data } = await apiInstance.koraHealthCheck();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**KoraHealthCheck200Response**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Service is healthy |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

