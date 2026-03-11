# VeraPointsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**claimSwap**](#claimswap) | **POST** /v1/vera/swap/claim | /vera/swap/claim|
|[**claimVault**](#claimvault) | **POST** /v1/vera/vault/claim | /vera/vault/claim|
|[**getBalance**](#getbalance) | **GET** /v1/vera/balance | Get user\&#39;s points, tier, and rank|
|[**getLeaderboard**](#getleaderboard) | **GET** /v1/vera/leaderboard | Top users by lifetime points|
|[**ping**](#ping) | **POST** /v1/vera/ping | Public health/liveness check (no auth)|
|[**recordSession**](#recordsession) | **POST** /v1/vera/session | /vera/session|

# **claimSwap**
> SwapClaimResponse claimSwap(swapClaimRequest)

Claim a swap transaction for Vera Points attribution. 

### Example

```typescript
import {
    VeraPointsApi,
    Configuration,
    SwapClaimRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new VeraPointsApi(configuration);

let swapClaimRequest: SwapClaimRequest; //

const { status, data } = await apiInstance.claimSwap(
    swapClaimRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **swapClaimRequest** | **SwapClaimRequest**|  | |


### Return type

**SwapClaimResponse**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Swap claim recorded successfully |  -  |
|**400** | Invalid or missing transaction hash |  -  |
|**401** | Missing or invalid authentication token |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **claimVault**
> VaultClaimResponse claimVault(vaultClaimRequest)

Claim a vault deposit transaction for Vera Points attribution. 

### Example

```typescript
import {
    VeraPointsApi,
    Configuration,
    VaultClaimRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new VeraPointsApi(configuration);

let vaultClaimRequest: VaultClaimRequest; //

const { status, data } = await apiInstance.claimVault(
    vaultClaimRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **vaultClaimRequest** | **VaultClaimRequest**|  | |


### Return type

**VaultClaimResponse**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Vault claim recorded successfully |  -  |
|**400** | Invalid or missing transaction hash |  -  |
|**401** | Missing or invalid authentication token |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getBalance**
> BalanceResponse getBalance()

Returns a user\'s lifetime Vera Points, current tier, rank, and next tier threshold. Public endpoint; user_address is passed as query parameter. 

### Example

```typescript
import {
    VeraPointsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new VeraPointsApi(configuration);

let userAddress: string; //Wallet address to look up balance for. (default to undefined)

const { status, data } = await apiInstance.getBalance(
    userAddress
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userAddress** | [**string**] | Wallet address to look up balance for. | defaults to undefined|


### Return type

**BalanceResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | User\&#39;s balance and tier info |  -  |
|**400** | Missing or invalid userAddress |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getLeaderboard**
> LeaderboardResponse getLeaderboard()

Paginated leaderboard ranked by lifetime Vera Points.

### Example

```typescript
import {
    VeraPointsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new VeraPointsApi(configuration);

let limit: number; //Number of entries to return. (optional) (default to 20)
let offset: number; //Number of entries to skip. (optional) (default to 0)

const { status, data } = await apiInstance.getLeaderboard(
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **limit** | [**number**] | Number of entries to return. | (optional) defaults to 20|
| **offset** | [**number**] | Number of entries to skip. | (optional) defaults to 0|


### Return type

**LeaderboardResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Leaderboard entries |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ping**
> PingResponse ping()

Public POST endpoint. No authentication required. Optional body for client identification; returns 200 with status ok. 

### Example

```typescript
import {
    VeraPointsApi,
    Configuration,
    PingRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new VeraPointsApi(configuration);

let pingRequest: PingRequest; // (optional)

const { status, data } = await apiInstance.ping(
    pingRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **pingRequest** | **PingRequest**|  | |


### Return type

**PingResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Service is alive |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **recordSession**
> SessionResponse recordSession()

Record a daily app session for vault points eligibility. user_address is extracted from JWT; session_date is set server-side to the current UTC date. Idempotent — one session per (user, date) pair; multiple calls on the same day are no-ops. This is a required daily gate for vault balance points. 

### Example

```typescript
import {
    VeraPointsApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new VeraPointsApi(configuration);

const { status, data } = await apiInstance.recordSession();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**SessionResponse**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Session recorded successfully |  -  |
|**401** | Missing or invalid authentication token |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

