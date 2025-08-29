# AccountDataApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**getAccountDetails**](#getaccountdetails) | **GET** /api/v1/account | /account|
|[**getAccountFundingRateHistory**](#getaccountfundingratehistory) | **GET** /api/v1/account/fundingRateHistory | /account/fundingRateHistory|
|[**getAccountPreferences**](#getaccountpreferences) | **GET** /api/v1/account/preferences | /account/preferences|
|[**getAccountTrades**](#getaccounttrades) | **GET** /api/v1/account/trades | /account/trades|
|[**getAccountTransactionHistory**](#getaccounttransactionhistory) | **GET** /api/v1/account/transactions | /account/transactions|
|[**patchAccountGroupID**](#patchaccountgroupid) | **PATCH** /api/v1/account/groupId | Set the group ID for an account.|
|[**putAccountPreferences**](#putaccountpreferences) | **PUT** /api/v1/account/preferences | /account/preferences|
|[**sponsorTx**](#sponsortx) | **POST** /api/v1/account/sponsorTx | /account/sponsorTx|

# **getAccountDetails**
> Account getAccountDetails()

Retrieves the user\'s account details.

### Example

```typescript
import {
    AccountDataApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new AccountDataApi(configuration);

let accountAddress: string; //Account address to fetch account details by. (optional) (default to undefined)

const { status, data } = await apiInstance.getAccountDetails(
    accountAddress
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **accountAddress** | [**string**] | Account address to fetch account details by. | (optional) defaults to undefined|


### Return type

**Account**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response with account details. |  -  |
|**400** | request missing required parameters |  -  |
|**401** | unauthorized access |  -  |
|**404** | account not found |  -  |
|**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAccountFundingRateHistory**
> AccountFundingRateHistory getAccountFundingRateHistory()

Retrieves the funding rate history for a specific account.

### Example

```typescript
import {
    AccountDataApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new AccountDataApi(configuration);

let accountAddress: string; //Account address to filter funding rate history by. (optional) (default to undefined)
let limit: number; //Default 500; max 1000. (optional) (default to 500)
let page: number; //The page number to retrieve in a paginated response. (optional) (default to 1)

const { status, data } = await apiInstance.getAccountFundingRateHistory(
    accountAddress,
    limit,
    page
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **accountAddress** | [**string**] | Account address to filter funding rate history by. | (optional) defaults to undefined|
| **limit** | [**number**] | Default 500; max 1000. | (optional) defaults to 500|
| **page** | [**number**] | The page number to retrieve in a paginated response. | (optional) defaults to 1|


### Return type

**AccountFundingRateHistory**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response with funding rate history. |  -  |
|**400** | request missing required parameters |  -  |
|**401** | unauthorized access |  -  |
|**404** | account not found |  -  |
|**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAccountPreferences**
> AccountPreference getAccountPreferences()

Retrieves the user\'s account preferences.

### Example

```typescript
import {
    AccountDataApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new AccountDataApi(configuration);

const { status, data } = await apiInstance.getAccountPreferences();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**AccountPreference**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response with account preferences. |  -  |
|**400** | request missing required parameters |  -  |
|**401** | unauthorized access |  -  |
|**404** | account not found |  -  |
|**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAccountTrades**
> Array<Trade> getAccountTrades()

Retrieves the user\'s trade history.

### Example

```typescript
import {
    AccountDataApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new AccountDataApi(configuration);

let symbol: string; //Market address to filter trades by. If not specified, returns trades for all markets. (optional) (default to undefined)
let startTimeAtMillis: number; //Start time in milliseconds. Defaults to 7 days ago if not specified. (optional) (default to undefined)
let endTimeAtMillis: number; //End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 7 days apart. (optional) (default to undefined)
let limit: number; //Default 500; max 1000. (optional) (default to 500)
let tradeType: TradeType; //Type of trade. By default returns all. UNSPECIFIED returns all. (optional) (default to undefined)
let page: number; //The page number to retrieve in a paginated response. (optional) (default to undefined)

const { status, data } = await apiInstance.getAccountTrades(
    symbol,
    startTimeAtMillis,
    endTimeAtMillis,
    limit,
    tradeType,
    page
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **symbol** | [**string**] | Market address to filter trades by. If not specified, returns trades for all markets. | (optional) defaults to undefined|
| **startTimeAtMillis** | [**number**] | Start time in milliseconds. Defaults to 7 days ago if not specified. | (optional) defaults to undefined|
| **endTimeAtMillis** | [**number**] | End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 7 days apart. | (optional) defaults to undefined|
| **limit** | [**number**] | Default 500; max 1000. | (optional) defaults to 500|
| **tradeType** | **TradeType** | Type of trade. By default returns all. UNSPECIFIED returns all. | (optional) defaults to undefined|
| **page** | [**number**] | The page number to retrieve in a paginated response. | (optional) defaults to undefined|


### Return type

**Array<Trade>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response with trade details. |  -  |
|**400** | request missing required parameters |  -  |
|**401** | unauthorized access |  -  |
|**404** | account not found |  -  |
|**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAccountTransactionHistory**
> Array<Transaction> getAccountTransactionHistory()

Retrieves the user\'s transaction history (any change in balance).

### Example

```typescript
import {
    AccountDataApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new AccountDataApi(configuration);

let types: Array<TransactionType>; //Optional query parameter to filter transactions by type. (optional) (default to undefined)
let assetSymbol: string; //Optional query parameter to filter transactions by asset bank address. (optional) (default to undefined)
let startTimeAtMillis: number; //Start time in milliseconds. Defaults to 7 days ago if not specified. (optional) (default to undefined)
let endTimeAtMillis: number; //End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 7 days apart. (optional) (default to undefined)
let limit: number; //Default 500; max 1000. (optional) (default to 500)
let page: number; //The page number to retrieve in a paginated response. (optional) (default to undefined)

const { status, data } = await apiInstance.getAccountTransactionHistory(
    types,
    assetSymbol,
    startTimeAtMillis,
    endTimeAtMillis,
    limit,
    page
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **types** | **Array&lt;TransactionType&gt;** | Optional query parameter to filter transactions by type. | (optional) defaults to undefined|
| **assetSymbol** | [**string**] | Optional query parameter to filter transactions by asset bank address. | (optional) defaults to undefined|
| **startTimeAtMillis** | [**number**] | Start time in milliseconds. Defaults to 7 days ago if not specified. | (optional) defaults to undefined|
| **endTimeAtMillis** | [**number**] | End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 7 days apart. | (optional) defaults to undefined|
| **limit** | [**number**] | Default 500; max 1000. | (optional) defaults to 500|
| **page** | [**number**] | The page number to retrieve in a paginated response. | (optional) defaults to undefined|


### Return type

**Array<Transaction>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response with transaction history details. |  -  |
|**400** | request missing required parameters |  -  |
|**401** | unauthorized access |  -  |
|**404** | account not found |  -  |
|**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patchAccountGroupID**
> patchAccountGroupID(accountGroupIdPatch)

Sets or updates the group ID for a specific account.  Accounts belonging to the same group cannot trade against each other. If the groupId is not set, the account will be removed from its group. 

### Example

```typescript
import {
    AccountDataApi,
    Configuration,
    AccountGroupIdPatch
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new AccountDataApi(configuration);

let accountGroupIdPatch: AccountGroupIdPatch; //Account group ID update.

const { status, data } = await apiInstance.patchAccountGroupID(
    accountGroupIdPatch
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **accountGroupIdPatch** | **AccountGroupIdPatch**| Account group ID update. | |


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
|**202** | Accepted and processing. |  -  |
|**400** | request missing required parameters |  -  |
|**401** | unauthorized access |  -  |
|**404** | account not found |  -  |
|**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **putAccountPreferences**
> putAccountPreferences(updateAccountPreferenceRequest)

Update user\'s account preferences. This will overwrite the preferences, so always send the full object.

### Example

```typescript
import {
    AccountDataApi,
    Configuration,
    UpdateAccountPreferenceRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new AccountDataApi(configuration);

let updateAccountPreferenceRequest: UpdateAccountPreferenceRequest; //

const { status, data } = await apiInstance.putAccountPreferences(
    updateAccountPreferenceRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **updateAccountPreferenceRequest** | **UpdateAccountPreferenceRequest**|  | |


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
|**204** | Success. |  -  |
|**400** | request missing request body |  -  |
|**401** | unauthorized access |  -  |
|**404** | account not found |  -  |
|**413** | request entity too large |  -  |
|**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sponsorTx**
> SponsorTxResponse sponsorTx(sponsorTxRequest)

Sponsors a transaction if it\'s eligible for sponsorship based on allowlisted methods and kinds.

### Example

```typescript
import {
    AccountDataApi,
    Configuration,
    SponsorTxRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new AccountDataApi(configuration);

let sponsorTxRequest: SponsorTxRequest; //

const { status, data } = await apiInstance.sponsorTx(
    sponsorTxRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **sponsorTxRequest** | **SponsorTxRequest**|  | |


### Return type

**SponsorTxResponse**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Transaction successfully sponsored. |  -  |
|**400** | Invalid request - transaction not eligible for sponsorship or missing required parameters |  -  |
|**401** | Unauthorized access - missing or invalid authentication token |  -  |
|**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

