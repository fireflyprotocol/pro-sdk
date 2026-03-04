# KoraApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**koraHealthCheck**](#korahealthcheck) | **GET** /v1/kora/health | Kora service health check|

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

