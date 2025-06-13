# AuthApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**authJwksGet**](#authjwksget) | **GET** /auth/jwks | |
|[**authTokenPost**](#authtokenpost) | **POST** /auth/token | |
|[**authTokenRefreshPut**](#authtokenrefreshput) | **PUT** /auth/token/refresh | |
|[**authV2TokenPost**](#authv2tokenpost) | **POST** /auth/v2/token | |

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
let refreshTokenValidForSeconds: number; //The number of seconds the refresh token is valid for. If not provided, the default is 30 days. (optional) (default to undefined)
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
| **refreshTokenValidForSeconds** | [**number**] | The number of seconds the refresh token is valid for. If not provided, the default is 30 days. | (optional) defaults to undefined|
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
let refreshTokenValidForSeconds: number; //The number of seconds the refresh token is valid for. If not provided, the default is 30 days. (optional) (default to undefined)
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
| **refreshTokenValidForSeconds** | [**number**] | The number of seconds the refresh token is valid for. If not provided, the default is 30 days. | (optional) defaults to undefined|
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

