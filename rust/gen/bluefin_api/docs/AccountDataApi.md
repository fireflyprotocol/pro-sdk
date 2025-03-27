# \AccountDataApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_details**](AccountDataApi.md#get_account_details) | **GET** /api/v1/account | Get user's account details.
[**get_account_funding_rate_history**](AccountDataApi.md#get_account_funding_rate_history) | **GET** /api/v1/account/fundingRateHistory | Get user's funding rate history
[**get_account_preferences**](AccountDataApi.md#get_account_preferences) | **GET** /api/v1/account/preferences | Get user's account preferences.
[**get_account_trades**](AccountDataApi.md#get_account_trades) | **GET** /api/v1/account/trades | Get user's trade history.
[**get_account_transaction_history**](AccountDataApi.md#get_account_transaction_history) | **GET** /api/v1/account/transactions | Get user's transaction history (any change in balance).



## get_account_details

> models::Account get_account_details()
Get user's account details.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Account**](Account.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_funding_rate_history

> models::AccountFundingRateHistory get_account_funding_rate_history(account_address, limit, page)
Get user's funding rate history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_address** | Option<**String**> | Account address to filter funding rate history by. |  |
**limit** | Option<**u32**> | Default 500; max 1000. |  |[default to 500]
**page** | Option<**u32**> | The page number to retrieve in a paginated response. |  |[default to 1]

### Return type

[**models::AccountFundingRateHistory**](AccountFundingRateHistory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_preferences

> models::AccountPreference get_account_preferences()
Get user's account preferences.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccountPreference**](AccountPreference.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_trades

> Vec<models::Trade> get_account_trades(symbol, start_time_at_millis, end_time_at_millis, limit, trade_type, page)
Get user's trade history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Market address to filter trades by. If not specified, returns trades for all markets. |  |
**start_time_at_millis** | Option<**u32**> | Start time in milliseconds. Defaults to 7 days ago if not specified. |  |
**end_time_at_millis** | Option<**u32**> | End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 7 days apart. |  |
**limit** | Option<**u32**> | Default 500; max 1000. |  |[default to 500]
**trade_type** | Option<[**TradeType**](.md)> | Type of trade. By default returns all. UNSPECIFIED returns all. |  |
**page** | Option<**u32**> | The page number to retrieve in a paginated response. |  |

### Return type

[**Vec<models::Trade>**](Trade.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_transaction_history

> Vec<models::Transaction> get_account_transaction_history(types, asset_symbol, start_time_at_millis, end_time_at_millis, limit, page)
Get user's transaction history (any change in balance).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**types** | Option<[**Vec<models::TransactionType>**](models::TransactionType.md)> | Optional query parameter to filter transactions by type. |  |
**asset_symbol** | Option<**String**> | Optional query parameter to filter transactions by asset bank address. |  |
**start_time_at_millis** | Option<**u32**> | Start time in milliseconds. Defaults to 7 days ago if not specified. |  |
**end_time_at_millis** | Option<**u32**> | End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 7 days apart. |  |
**limit** | Option<**u32**> | Default 500; max 1000. |  |[default to 500]
**page** | Option<**u32**> | The page number to retrieve in a paginated response. |  |

### Return type

[**Vec<models::Transaction>**](Transaction.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

