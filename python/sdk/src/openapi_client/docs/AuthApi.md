# openapi_client.AuthApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_jwks_get**](AuthApi.md#auth_jwks_get) | **GET** /auth/jwks | 
[**auth_token_post**](AuthApi.md#auth_token_post) | **POST** /auth/token | 
[**auth_token_refresh_put**](AuthApi.md#auth_token_refresh_put) | **PUT** /auth/token/refresh | 
[**auth_v2_token_post**](AuthApi.md#auth_v2_token_post) | **POST** /auth/v2/token | 
[**get_zk_login_user_details**](AuthApi.md#get_zk_login_user_details) | **GET** /auth/zklogin | /auth/zklogin
[**post_zk_login_zkp**](AuthApi.md#post_zk_login_zkp) | **POST** /auth/zklogin/zkp | /auth/zklogin/zkp


# **auth_jwks_get**
> Dict[str, object] auth_jwks_get()

### Example


```python
import openapi_client
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
    api_instance = openapi_client.AuthApi(api_client)

    try:
        api_response = await api_instance.auth_jwks_get()
        print("The response of AuthApi->auth_jwks_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AuthApi->auth_jwks_get: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

**Dict[str, object]**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | OK |  -  |
**0** | Unexpected error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **auth_token_post**
> LoginResponse auth_token_post(payload_signature, login_request, refresh_token_valid_for_seconds=refresh_token_valid_for_seconds, read_only=read_only)

login with token

### Example


```python
import openapi_client
from openapi_client.models.login_request import LoginRequest
from openapi_client.models.login_response import LoginResponse
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
    api_instance = openapi_client.AuthApi(api_client)
    payload_signature = 'payload_signature_example' # str | 
    login_request = openapi_client.LoginRequest() # LoginRequest | 
    refresh_token_valid_for_seconds = 2592000 # int | The number of seconds the refresh token is valid for. If not provided, the default is 30 days. (optional) (default to 2592000)
    read_only = False # bool |  (optional) (default to False)

    try:
        api_response = await api_instance.auth_token_post(payload_signature, login_request, refresh_token_valid_for_seconds=refresh_token_valid_for_seconds, read_only=read_only)
        print("The response of AuthApi->auth_token_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AuthApi->auth_token_post: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **payload_signature** | **str**|  | 
 **login_request** | [**LoginRequest**](LoginRequest.md)|  | 
 **refresh_token_valid_for_seconds** | **int**| The number of seconds the refresh token is valid for. If not provided, the default is 30 days. | [optional] [default to 2592000]
 **read_only** | **bool**|  | [optional] [default to False]

### Return type

[**LoginResponse**](LoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | OK |  -  |
**400** | bad signature |  -  |
**401** | invalid signature |  -  |
**403** | invalid audience |  -  |
**500** | internal server error |  -  |
**0** | Unexpected error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **auth_token_refresh_put**
> RefreshTokenResponse auth_token_refresh_put(refresh_token_request)

Retrieves a new auth token for an account. Expiry is set to 5 min.

### Example


```python
import openapi_client
from openapi_client.models.refresh_token_request import RefreshTokenRequest
from openapi_client.models.refresh_token_response import RefreshTokenResponse
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
    api_instance = openapi_client.AuthApi(api_client)
    refresh_token_request = openapi_client.RefreshTokenRequest() # RefreshTokenRequest | 

    try:
        api_response = await api_instance.auth_token_refresh_put(refresh_token_request)
        print("The response of AuthApi->auth_token_refresh_put:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AuthApi->auth_token_refresh_put: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **refresh_token_request** | [**RefreshTokenRequest**](RefreshTokenRequest.md)|  | 

### Return type

[**RefreshTokenResponse**](RefreshTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | OK |  -  |
**401** | invalid signature |  -  |
**400** | missing refresh token in request |  -  |
**500** | internal server error |  -  |
**0** | Unexpected error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **auth_v2_token_post**
> LoginResponse auth_v2_token_post(payload_signature, login_request, refresh_token_valid_for_seconds=refresh_token_valid_for_seconds, read_only=read_only)

login compatible with BCS payload with intent bytes

### Example


```python
import openapi_client
from openapi_client.models.login_request import LoginRequest
from openapi_client.models.login_response import LoginResponse
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
    api_instance = openapi_client.AuthApi(api_client)
    payload_signature = 'payload_signature_example' # str | 
    login_request = openapi_client.LoginRequest() # LoginRequest | 
    refresh_token_valid_for_seconds = 2592000 # int | The number of seconds the refresh token is valid for. If not provided, the default is 30 days. (optional) (default to 2592000)
    read_only = False # bool |  (optional) (default to False)

    try:
        api_response = await api_instance.auth_v2_token_post(payload_signature, login_request, refresh_token_valid_for_seconds=refresh_token_valid_for_seconds, read_only=read_only)
        print("The response of AuthApi->auth_v2_token_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AuthApi->auth_v2_token_post: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **payload_signature** | **str**|  | 
 **login_request** | [**LoginRequest**](LoginRequest.md)|  | 
 **refresh_token_valid_for_seconds** | **int**| The number of seconds the refresh token is valid for. If not provided, the default is 30 days. | [optional] [default to 2592000]
 **read_only** | **bool**|  | [optional] [default to False]

### Return type

[**LoginResponse**](LoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | OK |  -  |
**400** | bad signature |  -  |
**401** | invalid signature |  -  |
**403** | invalid audience |  -  |
**500** | internal server error |  -  |
**0** | Unexpected error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_zk_login_user_details**
> ZKLoginUserDetailsResponse get_zk_login_user_details(zklogin_jwt)

/auth/zklogin

ZK Login User Details

### Example


```python
import openapi_client
from openapi_client.models.zk_login_user_details_response import ZKLoginUserDetailsResponse
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
    api_instance = openapi_client.AuthApi(api_client)
    zklogin_jwt = 'zklogin_jwt_example' # str | The JWT of the user signed in with zkLogin.

    try:
        # /auth/zklogin
        api_response = await api_instance.get_zk_login_user_details(zklogin_jwt)
        print("The response of AuthApi->get_zk_login_user_details:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AuthApi->get_zk_login_user_details: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **zklogin_jwt** | **str**| The JWT of the user signed in with zkLogin. | 

### Return type

[**ZKLoginUserDetailsResponse**](ZKLoginUserDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response with zkLogin user details |  -  |
**400** | Bad Request |  -  |
**500** | internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_zk_login_zkp**
> ZKLoginZKPResponse post_zk_login_zkp(zklogin_jwt, zk_login_zkp_request)

/auth/zklogin/zkp

### Example


```python
import openapi_client
from openapi_client.models.zk_login_zkp_request import ZKLoginZKPRequest
from openapi_client.models.zk_login_zkp_response import ZKLoginZKPResponse
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
    api_instance = openapi_client.AuthApi(api_client)
    zklogin_jwt = 'zklogin_jwt_example' # str | The JWT of the user signed in with zkLogin.
    zk_login_zkp_request = openapi_client.ZKLoginZKPRequest() # ZKLoginZKPRequest | 

    try:
        # /auth/zklogin/zkp
        api_response = await api_instance.post_zk_login_zkp(zklogin_jwt, zk_login_zkp_request)
        print("The response of AuthApi->post_zk_login_zkp:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling AuthApi->post_zk_login_zkp: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **zklogin_jwt** | **str**| The JWT of the user signed in with zkLogin. | 
 **zk_login_zkp_request** | [**ZKLoginZKPRequest**](ZKLoginZKPRequest.md)|  | 

### Return type

[**ZKLoginZKPResponse**](ZKLoginZKPResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response with ZK proof result |  -  |
**400** | Bad Request |  -  |
**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

