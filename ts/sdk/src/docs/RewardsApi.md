# RewardsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**getAffiliateIntervalOverview**](#getaffiliateintervaloverview) | **GET** /v1/rewards/affiliate/intervalOverview | Get affiliate earnings overview by interval|
|[**getAffiliateLeaderDashboard**](#getaffiliateleaderdashboard) | **GET** /v1/rewards/affiliate/leaderDashboard | Get affiliate rankings and earnings|
|[**getAffiliateMetadata**](#getaffiliatemetadata) | **GET** /v1/rewards/affiliate | Get affiliate metadata|
|[**getAffiliateOverview**](#getaffiliateoverview) | **GET** /v1/rewards/affiliate/overview | Get detailed affiliate earnings overview|
|[**getAffiliateSummary**](#getaffiliatesummary) | **GET** /v1/rewards/affiliate/summary | Get affiliate performance summary|
|[**getCampaignRewards**](#getcampaignrewards) | **GET** /v1/rewards/campaign | Get rewards information for a specific campaign|
|[**getRewards**](#getrewards) | **GET** /v1/rewards | Get rewards information for the intervals|
|[**getRewardsCampaignMetadata**](#getrewardscampaignmetadata) | **GET** /v1/rewards/metadata/campaign | Get rewards metadata for the campaigns|
|[**getRewardsEpochConfigMetadata**](#getrewardsepochconfigmetadata) | **GET** /v1/rewards/metadata/epoch/configs | Gets the latest epoch configs for the campaigns|
|[**getRewardsEpochMetadata**](#getrewardsepochmetadata) | **GET** /v1/rewards/metadata/epoch | Gets the latest or next epoch for campaign.|
|[**getRewardsIntervalMetadata**](#getrewardsintervalmetadata) | **GET** /v1/rewards/metadata/interval | Gets the interval metadata for provided parameters|
|[**getRewardsSummary**](#getrewardssummary) | **GET** /v1/rewards/summary | Get rewards information for all time rewards earned|
|[**onboardAffiliate**](#onboardaffiliate) | **POST** /v1/rewards/affiliate/onboard | Submit affiliate onboarding application|
|[**onboardReferee**](#onboardreferee) | **POST** /v1/rewards/affiliate/onboard/referee | Onboard referee with a referral code|
|[**updateAffiliateFeeConfig**](#updateaffiliatefeeconfig) | **POST** /v1/rewards/affiliate/feeConfig | Update affiliate fee config|

# **getAffiliateIntervalOverview**
> GetAffiliateIntervalOverview200Response getAffiliateIntervalOverview()

Returns detailed earnings breakdown for an affiliate by interval, ordered by interval number in descending order

### Example

```typescript
import {
    RewardsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

let userAddress: string; //The address of the user to get interval overview for (default to undefined)
let page: number; //The page number to retrieve in a paginated response (optional) (default to 1)
let limit: number; //The page size for pagination (optional) (default to 500)

const { status, data } = await apiInstance.getAffiliateIntervalOverview(
    userAddress,
    page,
    limit
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userAddress** | [**string**] | The address of the user to get interval overview for | defaults to undefined|
| **page** | [**number**] | The page number to retrieve in a paginated response | (optional) defaults to 1|
| **limit** | [**number**] | The page size for pagination | (optional) defaults to 500|


### Return type

**GetAffiliateIntervalOverview200Response**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response array of affiliate interval overview objects |  -  |
|**401** | unauthorized access |  -  |
|**404** | address not found |  -  |
|**400** | request missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAffiliateLeaderDashboard**
> GetAffiliateLeaderDashboard200Response getAffiliateLeaderDashboard()

Returns rankings and earnings for affiliates, sorted by the specified category

### Example

```typescript
import {
    RewardsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

let sortBy: 'perpsRank' | 'lendingRank' | 'spotRank'; //The category to sort rankings by (optional) (default to 'perpsRank')
let sortOrder: 'asc' | 'desc'; //The order to sort rankings by (optional) (default to undefined)
let page: number; //The page number to retrieve in a paginated response (optional) (default to 1)
let limit: number; //The page size for pagination (optional) (default to 500)
let search: string; //The name/address of the user to filter by (optional) (default to undefined)

const { status, data } = await apiInstance.getAffiliateLeaderDashboard(
    sortBy,
    sortOrder,
    page,
    limit,
    search
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **sortBy** | [**&#39;perpsRank&#39; | &#39;lendingRank&#39; | &#39;spotRank&#39;**]**Array<&#39;perpsRank&#39; &#124; &#39;lendingRank&#39; &#124; &#39;spotRank&#39;>** | The category to sort rankings by | (optional) defaults to 'perpsRank'|
| **sortOrder** | [**&#39;asc&#39; | &#39;desc&#39;**]**Array<&#39;asc&#39; &#124; &#39;desc&#39;>** | The order to sort rankings by | (optional) defaults to undefined|
| **page** | [**number**] | The page number to retrieve in a paginated response | (optional) defaults to 1|
| **limit** | [**number**] | The page size for pagination | (optional) defaults to 500|
| **search** | [**string**] | The name/address of the user to filter by | (optional) defaults to undefined|


### Return type

**GetAffiliateLeaderDashboard200Response**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response array of affiliate ranking and earnings objects |  -  |
|**401** | unauthorized access |  -  |
|**404** | address not found |  -  |
|**400** | request missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAffiliateMetadata**
> AffiliateMetadata getAffiliateMetadata()

Returns the affiliate metadata

### Example

```typescript
import {
    RewardsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

let userAddress: string; //Specify wallet address. (default to undefined)

const { status, data } = await apiInstance.getAffiliateMetadata(
    userAddress
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userAddress** | [**string**] | Specify wallet address. | defaults to undefined|


### Return type

**AffiliateMetadata**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |
|**401** | unauthorized access |  -  |
|**404** | address not found |  -  |
|**400** | request missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAffiliateOverview**
> GetAffiliateOverview200Response getAffiliateOverview()

Returns detailed earnings breakdown for an affiliate users earnings (including perps, spot LP, lending), referral earnings, and total earnings

### Example

```typescript
import {
    RewardsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

let userAddress: string; //Specify wallet address. (default to undefined)
let page: number; //The page number to retrieve in a paginated response (optional) (default to 1)
let limit: number; //The page size for pagination (optional) (default to 500)
let sortBy: 'refreeEarnings' | 'referralEarnings' | 'totalEarnings'; //The category to sort earnings by (optional) (default to 'totalEarnings')
let sortOrder: 'asc' | 'desc'; //The order to sort earnings by (optional) (default to undefined)
let search: string; //The name/address of the user to filter by (optional) (default to undefined)
let minEarningsE9: string; //The minimum earnings to filter by (optional) (default to undefined)

const { status, data } = await apiInstance.getAffiliateOverview(
    userAddress,
    page,
    limit,
    sortBy,
    sortOrder,
    search,
    minEarningsE9
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userAddress** | [**string**] | Specify wallet address. | defaults to undefined|
| **page** | [**number**] | The page number to retrieve in a paginated response | (optional) defaults to 1|
| **limit** | [**number**] | The page size for pagination | (optional) defaults to 500|
| **sortBy** | [**&#39;refreeEarnings&#39; | &#39;referralEarnings&#39; | &#39;totalEarnings&#39;**]**Array<&#39;refreeEarnings&#39; &#124; &#39;referralEarnings&#39; &#124; &#39;totalEarnings&#39;>** | The category to sort earnings by | (optional) defaults to 'totalEarnings'|
| **sortOrder** | [**&#39;asc&#39; | &#39;desc&#39;**]**Array<&#39;asc&#39; &#124; &#39;desc&#39;>** | The order to sort earnings by | (optional) defaults to undefined|
| **search** | [**string**] | The name/address of the user to filter by | (optional) defaults to undefined|
| **minEarningsE9** | [**string**] | The minimum earnings to filter by | (optional) defaults to undefined|


### Return type

**GetAffiliateOverview200Response**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response array of affiliate overview objects |  -  |
|**401** | unauthorized access |  -  |
|**404** | address not found |  -  |
|**400** | request missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAffiliateSummary**
> AffiliateSummary getAffiliateSummary()

Returns performance summary for an affiliate including total referrals, earnings, and rankings

### Example

```typescript
import {
    RewardsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

let userAddress: string; //Specify wallet address. (default to undefined)

const { status, data } = await apiInstance.getAffiliateSummary(
    userAddress
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userAddress** | [**string**] | Specify wallet address. | defaults to undefined|


### Return type

**AffiliateSummary**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response of affiliate summary |  -  |
|**401** | unauthorized access |  -  |
|**404** | address not found |  -  |
|**400** | request missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getCampaignRewards**
> Array<UserCampaignRewards> getCampaignRewards()

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
let userAddress: string; //Specify wallet address. (default to undefined)
let epochNumber: number; //Optionally specify epoch number. (optional) (default to undefined)

const { status, data } = await apiInstance.getCampaignRewards(
    campaignName,
    userAddress,
    epochNumber
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **campaignName** | [**string**] | Specify the campaign name | defaults to undefined|
| **userAddress** | [**string**] | Specify wallet address. | defaults to undefined|
| **epochNumber** | [**number**] | Optionally specify epoch number. | (optional) defaults to undefined|


### Return type

**Array<UserCampaignRewards>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |
|**400** | request missing required parameters |  -  |

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

[bearerAuth](../README.md#bearerAuth)

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
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

let interval: number; //Either specify an interval number or the string \"next\" or \"latest\". (optional) (default to undefined)

const { status, data } = await apiInstance.getRewardsIntervalMetadata(
    interval
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **interval** | [**number**] | Either specify an interval number or the string \&quot;next\&quot; or \&quot;latest\&quot;. | (optional) defaults to undefined|


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

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **onboardAffiliate**
> AffiliateOnboardResponse onboardAffiliate(onboardAffiliateRequest)

Submit an application to become an affiliate

### Example

```typescript
import {
    RewardsApi,
    Configuration,
    OnboardAffiliateRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

let onboardAffiliateRequest: OnboardAffiliateRequest; //

const { status, data } = await apiInstance.onboardAffiliate(
    onboardAffiliateRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **onboardAffiliateRequest** | **OnboardAffiliateRequest**|  | |


### Return type

**AffiliateOnboardResponse**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Application submitted successfully |  -  |
|**401** | unauthorized access |  -  |
|**404** | address not found |  -  |
|**400** | request missing required parameters |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **onboardReferee**
> RefereeOnboardResponse onboardReferee(onboardRefereeRequest)

Onboard a referee with a referral code

### Example

```typescript
import {
    RewardsApi,
    Configuration,
    OnboardRefereeRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

let onboardRefereeRequest: OnboardRefereeRequest; //

const { status, data } = await apiInstance.onboardReferee(
    onboardRefereeRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **onboardRefereeRequest** | **OnboardRefereeRequest**|  | |


### Return type

**RefereeOnboardResponse**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Referee onboarding response |  -  |
|**401** | unauthorized access |  -  |
|**404** | parent referral code not found |  -  |
|**400** | referral code is required |  -  |
|**412** | parent affiliate is not active |  -  |
|**409** | referee already exists |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateAffiliateFeeConfig**
> AffiliateMetadata updateAffiliateFeeConfig(updateAffiliateFeeConfigRequest)

Update the fee config for an affiliate

### Example

```typescript
import {
    RewardsApi,
    Configuration,
    UpdateAffiliateFeeConfigRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new RewardsApi(configuration);

let updateAffiliateFeeConfigRequest: UpdateAffiliateFeeConfigRequest; //

const { status, data } = await apiInstance.updateAffiliateFeeConfig(
    updateAffiliateFeeConfigRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **updateAffiliateFeeConfigRequest** | **UpdateAffiliateFeeConfigRequest**|  | |


### Return type

**AffiliateMetadata**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Cashback request processed successfully |  -  |
|**401** | unauthorized access |  -  |
|**404** | address not found |  -  |
|**400** | request missing required parameters |  -  |
|**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

