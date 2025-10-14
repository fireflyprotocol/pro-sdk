# \AccountDataApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_details**](AccountDataApi.md#get_account_details) | **GET** /api/v1/account | /account
[**get_account_funding_rate_history**](AccountDataApi.md#get_account_funding_rate_history) | **GET** /api/v1/account/fundingRateHistory | /account/fundingRateHistory
[**get_account_preferences**](AccountDataApi.md#get_account_preferences) | **GET** /api/v1/account/preferences | /account/preferences
[**get_account_trades**](AccountDataApi.md#get_account_trades) | **GET** /api/v1/account/trades | /account/trades
[**get_account_transaction_history**](AccountDataApi.md#get_account_transaction_history) | **GET** /api/v1/account/transactions | /account/transactions
[**patch_account_group_id**](AccountDataApi.md#patch_account_group_id) | **PATCH** /api/v1/account/groupId | Set the group ID for an account.
[**put_account_preferences**](AccountDataApi.md#put_account_preferences) | **PUT** /api/v1/account/preferences | /account/preferences
[**sponsor_tx**](AccountDataApi.md#sponsor_tx) | **POST** /api/v1/account/sponsorTx | /account/sponsorTx



## get_account_details

> models::Account get_account_details(account_address)
/account

Retrieves the user's account details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_address** | Option<**String**> | Account address to fetch account details by. |  |

### Return type

[**models::Account**](Account.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_funding_rate_history

> models::AccountFundingRateHistory get_account_funding_rate_history(account_address, limit, page, start_time_at_millis, end_time_at_millis)
/account/fundingRateHistory

Retrieves the funding rate history for a specific account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_address** | Option<**String**> | Account address to filter funding rate history by. |  |
**limit** | Option<**u32**> | Default 500; max 1000. |  |[default to 500]
**page** | Option<**u32**> | The page number to retrieve in a paginated response. |  |[default to 1]
**start_time_at_millis** | Option<**u32**> | Start time in milliseconds. Defaults to 7 days ago if not specified. |  |
**end_time_at_millis** | Option<**u32**> | End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 90 days apart. |  |

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
/account/preferences

Retrieves the user's account preferences.

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
/account/trades

Retrieves the user's trade history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Market address to filter trades by. If not specified, returns trades for all markets. |  |
**start_time_at_millis** | Option<**u32**> | Start time in milliseconds. Defaults to 7 days ago if not specified. |  |
**end_time_at_millis** | Option<**u32**> | End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 90 days apart. |  |
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
/account/transactions

Retrieves the user's transaction history (any change in balance).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**types** | Option<[**Vec<models::TransactionType>**](models::TransactionType.md)> | Optional query parameter to filter transactions by type. |  |
**asset_symbol** | Option<**String**> | Optional query parameter to filter transactions by asset bank address. |  |
**start_time_at_millis** | Option<**u32**> | Start time in milliseconds. Defaults to 7 days ago if not specified. |  |
**end_time_at_millis** | Option<**u32**> | End time in milliseconds. Defaults to now if not specified. Must be greater than start time and must be less than 90 days apart. |  |
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


## patch_account_group_id

> patch_account_group_id(account_group_id_patch)
Set the group ID for an account.

Sets or updates the group ID for a specific account. Accounts belonging to the same group cannot trade against each other. If the groupId is not set, the account will be removed from its group. Only the first 6 characters of the groupID are guaranteed to be respected, longer group IDs may be rejected. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_group_id_patch** | [**AccountGroupIdPatch**](AccountGroupIdPatch.md) | Account group ID update. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_account_preferences

> put_account_preferences(update_account_preference_request)
/account/preferences

Update user's account preferences. This will overwrite the preferences, so always send the full object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_account_preference_request** | [**UpdateAccountPreferenceRequest**](UpdateAccountPreferenceRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sponsor_tx

> models::SponsorTxResponse sponsor_tx(sponsor_tx_request)
/account/sponsorTx

Sponsors a transaction if it's eligible for sponsorship based on allowlisted methods and kinds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sponsor_tx_request** | [**SponsorTxRequest**](SponsorTxRequest.md) |  | [required] |

### Return type

[**models::SponsorTxResponse**](SponsorTxResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

