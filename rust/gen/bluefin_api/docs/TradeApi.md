# \TradeApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_orders**](TradeApi.md#cancel_orders) | **PUT** /api/v1/trade/orders/cancel | Cancel orders for a market using order hashes
[**get_open_orders**](TradeApi.md#get_open_orders) | **GET** /api/v1/trade/openOrders | Get Open Orders
[**post_create_order**](TradeApi.md#post_create_order) | **POST** /api/v1/trade/orders | Create a new order
[**post_withdraw**](TradeApi.md#post_withdraw) | **POST** /api/v1/trade/withdraw | Initiate a withdraw
[**put_authorize_account**](TradeApi.md#put_authorize_account) | **PUT** /api/v1/trade/accounts/authorize | Authorizes an account
[**put_deauthorize_account**](TradeApi.md#put_deauthorize_account) | **PUT** /api/v1/trade/accounts/deauthorize | Deauthorizes an account
[**put_leverage_update**](TradeApi.md#put_leverage_update) | **PUT** /api/v1/trade/leverage | Updates leverage for positions



## cancel_orders

> cancel_orders(cancel_orders_request)
Cancel orders for a market using order hashes

- May be a single order hash or a list of order hashes. - All orders must belong to the same account. - If no order hashes are specified, then will cancel all orders for the given market  - All orders being cancelled by request will receive the same time priority. 

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


## get_open_orders

> Vec<models::OpenOrderResponse> get_open_orders(symbol)
Get Open Orders

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


## post_create_order

> models::PostCreateOrder202Response post_create_order(create_order_request)
Create a new order

Submit a new order for execution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_order_request** | [**CreateOrderRequest**](CreateOrderRequest.md) |  | [required] |

### Return type

[**models::PostCreateOrder202Response**](postCreateOrder_202_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_withdraw

> post_withdraw(withdraw_request)
Initiate a withdraw

Initiates a withdraw action to withdraw some amount of assets from a user's account

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


## put_authorize_account

> put_authorize_account(account_authorization_request)
Authorizes an account

Authorizes an account to trade, perform liquidations and more, on behalf of another account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_authorization_request** | [**AccountAuthorizationRequest**](AccountAuthorizationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_deauthorize_account

> put_deauthorize_account(account_authorization_request)
Deauthorizes an account

Deauthorizes an account to trade, perform liquidations and more, on behalf of another account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_authorization_request** | [**AccountAuthorizationRequest**](AccountAuthorizationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_leverage_update

> put_leverage_update(account_position_leverage_update_request)
Updates leverage for positions

Updates leverage for positions of a given market, closes all open orders for that market

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

