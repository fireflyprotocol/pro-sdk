# \StreamsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**web_socket_account_data**](StreamsApi.md#web_socket_account_data) | **GET** /ws/account | 
[**web_socket_market_data**](StreamsApi.md#web_socket_market_data) | **GET** /ws/market | 



## web_socket_account_data

> web_socket_account_data(authorization, upgrade, sec_web_socket_key, sec_web_socket_version)


WebSocket Account Streams URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**upgrade** | **String** |  | [required] |
**sec_web_socket_key** | **String** | WebSocket key used during the handshake. | [required] |
**sec_web_socket_version** | **String** | WebSocket protocol version. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## web_socket_market_data

> web_socket_market_data(upgrade, sec_web_socket_key, sec_web_socket_version)


WebSocket Market Streams URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upgrade** | **String** |  | [required] |
**sec_web_socket_key** | **String** | WebSocket key used during the handshake. | [required] |
**sec_web_socket_version** | **String** | WebSocket protocol version. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

