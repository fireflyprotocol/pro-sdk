# \VeraPointsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**claim_swap**](VeraPointsApi.md#claim_swap) | **POST** /v1/vera/swap/claim | /vera/swap/claim
[**claim_vault**](VeraPointsApi.md#claim_vault) | **POST** /v1/vera/vault/claim | /vera/vault/claim
[**get_balance**](VeraPointsApi.md#get_balance) | **GET** /v1/vera/balance | Get user's points, tier, and rank
[**get_leaderboard**](VeraPointsApi.md#get_leaderboard) | **GET** /v1/vera/leaderboard | Top users by lifetime points
[**ping**](VeraPointsApi.md#ping) | **POST** /v1/vera/ping | Public health/liveness check (no auth)
[**record_session**](VeraPointsApi.md#record_session) | **POST** /v1/vera/session | /vera/session



## claim_swap

> models::SwapClaimResponse claim_swap(swap_claim_request)
/vera/swap/claim

Claim a swap transaction for Vera Points attribution. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**swap_claim_request** | [**SwapClaimRequest**](SwapClaimRequest.md) |  | [required] |

### Return type

[**models::SwapClaimResponse**](SwapClaimResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## claim_vault

> models::VaultClaimResponse claim_vault(vault_claim_request)
/vera/vault/claim

Claim a vault deposit transaction for Vera Points attribution. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_claim_request** | [**VaultClaimRequest**](VaultClaimRequest.md) |  | [required] |

### Return type

[**models::VaultClaimResponse**](VaultClaimResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_balance

> models::BalanceResponse get_balance(user_address)
Get user's points, tier, and rank

Returns a user's lifetime Vera Points, current tier, rank, and next tier threshold. Public endpoint; user_address is passed as query parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_address** | **String** | Wallet address to look up balance for. | [required] |

### Return type

[**models::BalanceResponse**](BalanceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_leaderboard

> models::LeaderboardResponse get_leaderboard(limit, offset)
Top users by lifetime points

Paginated leaderboard ranked by lifetime Vera Points.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**u32**> | Number of entries to return. |  |[default to 20]
**offset** | Option<**u32**> | Number of entries to skip. |  |[default to 0]

### Return type

[**models::LeaderboardResponse**](LeaderboardResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ping

> models::PingResponse ping(ping_request)
Public health/liveness check (no auth)

Public POST endpoint. No authentication required. Optional body for client identification; returns 200 with status ok. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ping_request** | Option<[**PingRequest**](PingRequest.md)> |  |  |

### Return type

[**models::PingResponse**](PingResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## record_session

> models::SessionResponse record_session()
/vera/session

Record a daily app session for vault points eligibility. user_address is extracted from JWT; session_date is set server-side to the current UTC date. Idempotent — one session per (user, date) pair; multiple calls on the same day are no-ops. This is a required daily gate for vault balance points. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SessionResponse**](SessionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

