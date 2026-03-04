# openapi_client.KoraApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**kora_health_check**](KoraApi.md#kora_health_check) | **GET** /v1/kora/health | Kora service health check


# **kora_health_check**
> KoraHealthCheck200Response kora_health_check()

Kora service health check

Returns the health status of the Kora rewards service.

### Example


```python
import openapi_client
from openapi_client.models.kora_health_check200_response import KoraHealthCheck200Response
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
    api_instance = openapi_client.KoraApi(api_client)

    try:
        # Kora service health check
        api_response = await api_instance.kora_health_check()
        print("The response of KoraApi->kora_health_check:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling KoraApi->kora_health_check: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**KoraHealthCheck200Response**](KoraHealthCheck200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Service is healthy |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

