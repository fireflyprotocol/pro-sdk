# \AuthApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_jwks_get**](AuthApi.md#auth_jwks_get) | **GET** /auth/jwks | 
[**auth_token_post**](AuthApi.md#auth_token_post) | **POST** /auth/token | 
[**auth_token_refresh_put**](AuthApi.md#auth_token_refresh_put) | **PUT** /auth/token/refresh | 
[**auth_v2_token_post**](AuthApi.md#auth_v2_token_post) | **POST** /auth/v2/token | 



## auth_jwks_get

> std::collections::HashMap<String, serde_json::Value> auth_jwks_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_token_post

> models::LoginResponse auth_token_post(payload_signature, login_request, refresh_token_valid_for_seconds, read_only)


login with token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload_signature** | **String** |  | [required] |
**login_request** | [**LoginRequest**](LoginRequest.md) |  | [required] |
**refresh_token_valid_for_seconds** | Option<**i64**> | The number of seconds the refresh token is valid for. If not provided, the default is 30 days. |  |[default to 2592000]
**read_only** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_token_refresh_put

> models::RefreshTokenResponse auth_token_refresh_put(refresh_token_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refresh_token_request** | [**RefreshTokenRequest**](RefreshTokenRequest.md) |  | [required] |

### Return type

[**models::RefreshTokenResponse**](RefreshTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_v2_token_post

> models::LoginResponse auth_v2_token_post(payload_signature, login_request, refresh_token_valid_for_seconds, read_only)


login compatible with BCS payload with intent bytes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload_signature** | **String** |  | [required] |
**login_request** | [**LoginRequest**](LoginRequest.md) |  | [required] |
**refresh_token_valid_for_seconds** | Option<**i64**> | The number of seconds the refresh token is valid for. If not provided, the default is 30 days. |  |[default to 2592000]
**read_only** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

