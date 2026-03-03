# \KoraApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_kora_campaign_metadata**](KoraApi.md#get_kora_campaign_metadata) | **GET** /v1/kora/metadata/campaign | Get Kora campaign metadata
[**get_kora_campaign_rewards**](KoraApi.md#get_kora_campaign_rewards) | **GET** /v1/kora/rewards/campaign | Get Kora campaign rewards
[**get_kora_epoch_config_metadata**](KoraApi.md#get_kora_epoch_config_metadata) | **GET** /v1/kora/metadata/epoch/configs | Get Kora epoch configuration
[**get_kora_epoch_metadata**](KoraApi.md#get_kora_epoch_metadata) | **GET** /v1/kora/metadata/epoch | Get Kora epoch metadata
[**get_kora_interval_metadata**](KoraApi.md#get_kora_interval_metadata) | **GET** /v1/kora/metadata/interval | Get Kora interval metadata
[**get_kora_leaderboard**](KoraApi.md#get_kora_leaderboard) | **GET** /v1/kora/leaderboard | Get Kora swap leaderboard
[**get_kora_rewards_summary**](KoraApi.md#get_kora_rewards_summary) | **GET** /v1/kora/rewards/summary | Get Kora all-time rewards summary
[**kora_health_check**](KoraApi.md#kora_health_check) | **GET** /v1/kora/health | Kora service health check



## get_kora_campaign_metadata

> Vec<models::KoraCampaignMetadata> get_kora_campaign_metadata(campaign_name, status)
Get Kora campaign metadata

Returns metadata for Kora rewards campaigns.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_name** | Option<**String**> | Specify the campaign name |  |
**status** | Option<**String**> | Filter by campaign status |  |[default to ACTIVE]

### Return type

[**Vec<models::KoraCampaignMetadata>**](KoraCampaignMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_kora_campaign_rewards

> Vec<models::KoraCampaignRewards> get_kora_campaign_rewards(user_address, campaign_name, epoch_number)
Get Kora campaign rewards

Returns the rewards earned by users for Kora campaigns.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_address** | **String** | Specify wallet address | [required] |
**campaign_name** | Option<**String**> | Specify the campaign name |  |[default to KORA_SWAPS]
**epoch_number** | Option<**i32**> | Optionally specify epoch number |  |

### Return type

[**Vec<models::KoraCampaignRewards>**](KoraCampaignRewards.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_kora_epoch_config_metadata

> models::KoraEpochConfigResponse get_kora_epoch_config_metadata(interval_number, campaign_name)
Get Kora epoch configuration

Returns epoch configuration including reward allocations for Kora.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval_number** | Option<**i32**> | Specify the interval number |  |
**campaign_name** | Option<**String**> | Filter by campaign name |  |

### Return type

[**models::KoraEpochConfigResponse**](KoraEpochConfigResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_kora_epoch_metadata

> Vec<models::KoraEpochMetadata> get_kora_epoch_metadata(campaign_name, epoch)
Get Kora epoch metadata

Returns the latest or next epoch for a Kora campaign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_name** | Option<**String**> | Specify the campaign name |  |
**epoch** | Option<**String**> | Specify \"next\" or \"latest\" |  |

### Return type

[**Vec<models::KoraEpochMetadata>**](KoraEpochMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_kora_interval_metadata

> Vec<models::KoraIntervalMetadata> get_kora_interval_metadata(interval, protocol)
Get Kora interval metadata

Returns interval metadata for Kora.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval** | Option<**String**> | Interval number or \"next\"/\"latest\" |  |
**protocol** | Option<**String**> | Filter by protocol |  |[default to kora]

### Return type

[**Vec<models::KoraIntervalMetadata>**](KoraIntervalMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_kora_leaderboard

> models::KoraLeaderboardResponse get_kora_leaderboard(epoch_id, sort_by, sort_order, page, limit, search, min_volume_e6)
Get Kora swap leaderboard

Returns rankings and earnings for Kora swap participants, sorted by the specified category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epoch_id** | Option<**uuid::Uuid**> | Specify epoch ID (defaults to current active epoch) |  |
**sort_by** | Option<**String**> | The category to sort rankings by |  |[default to volumeRank]
**sort_order** | Option<**String**> | The order to sort rankings by |  |[default to desc]
**page** | Option<**u32**> | Page number for pagination |  |[default to 1]
**limit** | Option<**u32**> | Page size for pagination |  |[default to 50]
**search** | Option<**String**> | Filter by user address (partial match supported) |  |
**min_volume_e6** | Option<**String**> | Minimum trading volume filter (e6 format) |  |[default to 0]

### Return type

[**models::KoraLeaderboardResponse**](KoraLeaderboardResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_kora_rewards_summary

> Vec<models::KoraRewardsSummary> get_kora_rewards_summary()
Get Kora all-time rewards summary

Returns the all-time rewards earned by users including Kora CC and points.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::KoraRewardsSummary>**](KoraRewardsSummary.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kora_health_check

> models::KoraHealthCheck200Response kora_health_check()
Kora service health check

Returns the health status of the Kora rewards service.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::KoraHealthCheck200Response**](koraHealthCheck_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

