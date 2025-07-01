# \TradeApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_orders**](TradeApi.md#cancel_orders) | **PUT** /api/v1/trade/orders/cancel | /trade/orders/cancel
[**cancel_standby_orders**](TradeApi.md#cancel_standby_orders) | **PUT** /api/v1/trade/orders/cancel/standby | /trade/orders/cancel/standby
[**get_open_orders**](TradeApi.md#get_open_orders) | **GET** /api/v1/trade/openOrders | /trade/openOrders
[**get_standby_orders**](TradeApi.md#get_standby_orders) | **GET** /api/v1/trade/standbyOrders | /trade/standbyOrders
[**post_create_order**](TradeApi.md#post_create_order) | **POST** /api/v1/trade/orders | /trade/orders
[**post_withdraw**](TradeApi.md#post_withdraw) | **POST** /api/v1/trade/withdraw | /trade/withdraw
[**put_adjust_isolated_margin**](TradeApi.md#put_adjust_isolated_margin) | **PUT** /api/v1/trade/adjustIsolatedMargin | /trade/adjustIsolatedMargin
[**put_authorize_account**](TradeApi.md#put_authorize_account) | **PUT** /api/v1/trade/accounts/authorize | /trade/accounts/authorize
[**put_deauthorize_account**](TradeApi.md#put_deauthorize_account) | **PUT** /api/v1/trade/accounts/deauthorize | /trade/accounts/deauthorize
[**put_leverage_update**](TradeApi.md#put_leverage_update) | **PUT** /api/v1/trade/leverage | /trade/leverage



## cancel_orders

> cancel_orders(cancel_orders_request)
/trade/orders/cancel

Cancel orders for a market using order hashes. - May be a single order hash or a list of order hashes. - All orders must belong to the same account. - If no order hashes are specified, then will cancel all orders for the given market  - All orders being cancelled by request will receive the same time priority. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cancel_orders_request** | [**CancelOrdersRequest**](CancelOrdersRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_standby_orders

> models::CancelOrdersResponse cancel_standby_orders(cancel_orders_request)
/trade/orders/cancel/standby

Cancel orders in standby for a market using order hashes. - May be a single order hash or a list of order hashes. - All orders must belong to the same account. - If no order hashes are specified, then will cancel all orders for the given market - All orders being cancelled by request will receive the same time priority. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cancel_orders_request** | [**CancelOrdersRequest**](CancelOrdersRequest.md) |  | [required] |

### Return type

[**models::CancelOrdersResponse**](CancelOrdersResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_open_orders

> Vec<models::OpenOrderResponse> get_open_orders(symbol)
/trade/openOrders

Retrieve details of open orders for a specific account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Filter by specific perpetual symbol (optional) |  |

### Return type

[**Vec<models::OpenOrderResponse>**](OpenOrderResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_standby_orders

> Vec<models::OpenOrderResponse> get_standby_orders(symbol)
/trade/standbyOrders

Retrieve details of orders in standby for a specific account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Filter by specific perpetual symbol (optional) |  |

### Return type

[**Vec<models::OpenOrderResponse>**](OpenOrderResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_create_order

> models::CreateOrderResponse post_create_order(create_order_request)
/trade/orders

Submit a new order for execution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_order_request** | [**CreateOrderRequest**](CreateOrderRequest.md) |  | [required] |

### Return type

[**models::CreateOrderResponse**](CreateOrderResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_withdraw

> post_withdraw(withdraw_request)
/trade/withdraw

Initiates a withdraw action to remove some amount of funds from a user's account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**withdraw_request** | [**WithdrawRequest**](WithdrawRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_adjust_isolated_margin

> put_adjust_isolated_margin(adjust_isolated_margin_request)
/trade/adjustIsolatedMargin

Adjust margin for an isolated position on a specific market.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**adjust_isolated_margin_request** | [**AdjustIsolatedMarginRequest**](AdjustIsolatedMarginRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_authorize_account

> put_authorize_account(account_authorization_request)
/trade/accounts/authorize

Authorizes an account to trade, perform liquidations and more, on behalf of another account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_authorization_request** | [**AccountAuthorizationRequest**](AccountAuthorizationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_deauthorize_account

> put_deauthorize_account(account_authorization_request)
/trade/accounts/deauthorize

Deauthorizes an account to trade, perform liquidations and more, on behalf of another account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_authorization_request** | [**AccountAuthorizationRequest**](AccountAuthorizationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_leverage_update

> put_leverage_update(account_position_leverage_update_request)
/trade/leverage

Updates leverage for positions of a given market and closes all open orders for that market.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_position_leverage_update_request** | [**AccountPositionLeverageUpdateRequest**](AccountPositionLeverageUpdateRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

