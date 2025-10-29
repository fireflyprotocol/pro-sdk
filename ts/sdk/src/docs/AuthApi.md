# AuthApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**authJwksGet**](#authjwksget) | **GET** /auth/jwks | |
|[**authTokenPost**](#authtokenpost) | **POST** /auth/token | |
|[**authTokenRefreshPut**](#authtokenrefreshput) | **PUT** /auth/token/refresh | |
|[**authV2TokenPost**](#authv2tokenpost) | **POST** /auth/v2/token | |
|[**getZkLoginUserDetails**](#getzkloginuserdetails) | **GET** /auth/zklogin | /auth/zklogin|
|[**postZkLoginZkp**](#postzkloginzkp) | **POST** /auth/zklogin/zkp | /auth/zklogin/zkp|

# **authJwksGet**
> { [key: string]: any | undefined; } authJwksGet()


### Example

```typescript
import {
    AuthApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new AuthApi(configuration);

const { status, data } = await apiInstance.authJwksGet();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**{ [key: string]: any | undefined; }**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | OK |  -  |
|**0** | Unexpected error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **authTokenPost**
> LoginResponse authTokenPost(loginRequest)

login with token

### Example

```typescript
import {
    AuthApi,
    Configuration,
    LoginRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new AuthApi(configuration);

let payloadSignature: string; // (default to undefined)
let loginRequest: LoginRequest; //
let refreshTokenValidForSeconds: number; //The number of seconds the refresh token is valid for. If not provided, the default is 30 days. (optional) (default to 2592000)
let readOnly: boolean; // (optional) (default to false)

const { status, data } = await apiInstance.authTokenPost(
    payloadSignature,
    loginRequest,
    refreshTokenValidForSeconds,
    readOnly
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **loginRequest** | **LoginRequest**|  | |
| **payloadSignature** | [**string**] |  | defaults to undefined|
| **refreshTokenValidForSeconds** | [**number**] | The number of seconds the refresh token is valid for. If not provided, the default is 30 days. | (optional) defaults to 2592000|
| **readOnly** | [**boolean**] |  | (optional) defaults to false|


### Return type

**LoginResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | OK |  -  |
|**400** | bad signature |  -  |
|**401** | invalid signature |  -  |
|**403** | invalid audience |  -  |
|**500** | internal server error |  -  |
|**0** | Unexpected error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **authTokenRefreshPut**
> RefreshTokenResponse authTokenRefreshPut(refreshTokenRequest)

Retrieves a new auth token for an account. Expiry is set to 5 min.

### Example

```typescript
import {
    AuthApi,
    Configuration,
    RefreshTokenRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new AuthApi(configuration);

let refreshTokenRequest: RefreshTokenRequest; //

const { status, data } = await apiInstance.authTokenRefreshPut(
    refreshTokenRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **refreshTokenRequest** | **RefreshTokenRequest**|  | |


### Return type

**RefreshTokenResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | OK |  -  |
|**401** | invalid signature |  -  |
|**400** | missing refresh token in request |  -  |
|**500** | internal server error |  -  |
|**0** | Unexpected error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **authV2TokenPost**
> LoginResponse authV2TokenPost(loginRequest)

login compatible with BCS payload with intent bytes

### Example

```typescript
import {
    AuthApi,
    Configuration,
    LoginRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new AuthApi(configuration);

let payloadSignature: string; // (default to undefined)
let loginRequest: LoginRequest; //
let refreshTokenValidForSeconds: number; //The number of seconds the refresh token is valid for. If not provided, the default is 30 days. (optional) (default to 2592000)
let readOnly: boolean; // (optional) (default to false)

const { status, data } = await apiInstance.authV2TokenPost(
    payloadSignature,
    loginRequest,
    refreshTokenValidForSeconds,
    readOnly
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **loginRequest** | **LoginRequest**|  | |
| **payloadSignature** | [**string**] |  | defaults to undefined|
| **refreshTokenValidForSeconds** | [**number**] | The number of seconds the refresh token is valid for. If not provided, the default is 30 days. | (optional) defaults to 2592000|
| **readOnly** | [**boolean**] |  | (optional) defaults to false|


### Return type

**LoginResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | OK |  -  |
|**400** | bad signature |  -  |
|**401** | invalid signature |  -  |
|**403** | invalid audience |  -  |
|**500** | internal server error |  -  |
|**0** | Unexpected error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getZkLoginUserDetails**
> ZKLoginUserDetailsResponse getZkLoginUserDetails()

ZK Login User Details

### Example

```typescript
import {
    AuthApi,
    Configuration
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new AuthApi(configuration);

let zkloginJwt: string; //The JWT of the user signed in with zkLogin. (default to undefined)

const { status, data } = await apiInstance.getZkLoginUserDetails(
    zkloginJwt
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **zkloginJwt** | [**string**] | The JWT of the user signed in with zkLogin. | defaults to undefined|


### Return type

**ZKLoginUserDetailsResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response with zkLogin user details |  -  |
|**400** | Bad Request |  -  |
|**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postZkLoginZkp**
> ZKLoginZKPResponse postZkLoginZkp(zKLoginZKPRequest)


### Example

```typescript
import {
    AuthApi,
    Configuration,
    ZKLoginZKPRequest
} from '@bluefin/api-client';

const configuration = new Configuration();
const apiInstance = new AuthApi(configuration);

let zkloginJwt: string; //The JWT of the user signed in with zkLogin. (default to undefined)
let zKLoginZKPRequest: ZKLoginZKPRequest; //

const { status, data } = await apiInstance.postZkLoginZkp(
    zkloginJwt,
    zKLoginZKPRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **zKLoginZKPRequest** | **ZKLoginZKPRequest**|  | |
| **zkloginJwt** | [**string**] | The JWT of the user signed in with zkLogin. | defaults to undefined|


### Return type

**ZKLoginZKPResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response with ZK proof result |  -  |
|**400** | Bad Request |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

