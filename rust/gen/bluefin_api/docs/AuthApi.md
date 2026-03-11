# \AuthApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_jwks_get**](AuthApi.md#auth_jwks_get) | **GET** /auth/jwks | 
[**auth_token_post**](AuthApi.md#auth_token_post) | **POST** /auth/token | 
[**auth_token_refresh_put**](AuthApi.md#auth_token_refresh_put) | **PUT** /auth/token/refresh | 
[**auth_v2_token_post**](AuthApi.md#auth_v2_token_post) | **POST** /auth/v2/token | 
[**get_well_known_openid_configuration**](AuthApi.md#get_well_known_openid_configuration) | **GET** /.well-known/openid-configuration | 
[**get_zk_login_user_details**](AuthApi.md#get_zk_login_user_details) | **GET** /auth/zklogin | 
[**post_auth_client_credentials**](AuthApi.md#post_auth_client_credentials) | **POST** /auth/client-credentials | 
[**post_zk_login_zkp**](AuthApi.md#post_zk_login_zkp) | **POST** /auth/zklogin/zkp | ZK Login Zero-Knowledge Proof Proxy Endpoint



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

> models::LoginResponse auth_token_post(payload_signature, login_request, refresh_token_valid_for_seconds, read_only, client)


login with token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload_signature** | **String** |  | [required] |
**login_request** | [**LoginRequest**](LoginRequest.md) |  | [required] |
**refresh_token_valid_for_seconds** | Option<**i64**> | The number of seconds the refresh token is valid for. If not provided, the default is 30 days. |  |[default to 2592000]
**read_only** | Option<**bool**> |  |  |[default to false]
**client** | Option<[**ClientType**](.md)> | The client application originating the request (WEB or VERA). Defaults to WEB if not supplied. |  |

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


Retrieves a new auth token for an account. Expiry is set to 5 min.

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

> models::LoginResponse auth_v2_token_post(payload_signature, login_request, refresh_token_valid_for_seconds, read_only, client)


login compatible with BCS payload with intent bytes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload_signature** | **String** |  | [required] |
**login_request** | [**LoginRequest**](LoginRequest.md) |  | [required] |
**refresh_token_valid_for_seconds** | Option<**i64**> | The number of seconds the refresh token is valid for. If not provided, the default is 30 days. |  |[default to 2592000]
**read_only** | Option<**bool**> |  |  |[default to false]
**client** | Option<[**ClientType**](.md)> | The client application originating the request (WEB or VERA). Defaults to WEB if not supplied. |  |

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_well_known_openid_configuration

> models::OpenIdConfigurationResponse get_well_known_openid_configuration()


OpenID Connect Discovery endpoint

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OpenIdConfigurationResponse**](OpenIDConfigurationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zk_login_user_details

> models::ZkLoginUserDetailsResponse get_zk_login_user_details(zklogin_jwt)


ZK Login User Details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zklogin_jwt** | **String** | The JWT of the user signed in with zkLogin. | [required] |

### Return type

[**models::ZkLoginUserDetailsResponse**](ZKLoginUserDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_auth_client_credentials

> models::ClientCredentialsResponse post_auth_client_credentials(client_credentials_request)


OAuth2 client_credentials grant for service accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_credentials_request** | [**ClientCredentialsRequest**](ClientCredentialsRequest.md) |  | [required] |

### Return type

[**models::ClientCredentialsResponse**](ClientCredentialsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_zk_login_zkp

> models::ZkLoginZkpResponse post_zk_login_zkp(zklogin_jwt, zk_login_zkp_request)
ZK Login Zero-Knowledge Proof Proxy Endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zklogin_jwt** | **String** | The JWT of the user signed in with zkLogin. | [required] |
**zk_login_zkp_request** | [**ZkLoginZkpRequest**](ZkLoginZkpRequest.md) |  | [required] |

### Return type

[**models::ZkLoginZkpResponse**](ZKLoginZKPResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

