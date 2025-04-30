# \RewardsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_campaign_rewards**](RewardsApi.md#get_campaign_rewards) | **GET** /v1/rewards/campaign | Get rewards information for a specific campaign
[**get_rewards**](RewardsApi.md#get_rewards) | **GET** /v1/rewards | Get rewards information for the intervals
[**get_rewards_campaign_metadata**](RewardsApi.md#get_rewards_campaign_metadata) | **GET** /v1/rewards/metadata/campaign | Get rewards metadata for the campaigns
[**get_rewards_epoch_config_metadata**](RewardsApi.md#get_rewards_epoch_config_metadata) | **GET** /v1/rewards/metadata/epoch/configs | Gets the latest epoch configs for the campaigns
[**get_rewards_epoch_metadata**](RewardsApi.md#get_rewards_epoch_metadata) | **GET** /v1/rewards/metadata/epoch | Gets the latest or next epoch for campaign.
[**get_rewards_interval_metadata**](RewardsApi.md#get_rewards_interval_metadata) | **GET** /v1/rewards/metadata/interval | Gets the interval metadata for provided parameters
[**get_rewards_summary**](RewardsApi.md#get_rewards_summary) | **GET** /v1/rewards/summary | Get rewards information for all time rewards earned



## get_campaign_rewards

> Vec<models::CampaignRewards> get_campaign_rewards(campaign_name, epoch_number)
Get rewards information for a specific campaign

Returns the rewards earned by users for a specific campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_name** | **String** | Specify the campaign name | [required] |
**epoch_number** | Option<**i32**> | Optionally specify epoch number. |  |

### Return type

[**Vec<models::CampaignRewards>**](CampaignRewards.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rewards

> Vec<models::IntervalRewards> get_rewards(interval_number)
Get rewards information for the intervals

Returns the rewards earned by users for the intervals .

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval_number** | Option<**i32**> | Optionally specify interval number. |  |

### Return type

[**Vec<models::IntervalRewards>**](IntervalRewards.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rewards_campaign_metadata

> Vec<models::CampaignMetadata> get_rewards_campaign_metadata(campaign_name, status)
Get rewards metadata for the campaigns

Returns the metadata for the rewards campaigns.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_name** | Option<**String**> | Specify the campaign name |  |
**status** | Option<**String**> | Optionally specify the status of the campaigns. |  |[default to ACTIVE]

### Return type

[**Vec<models::CampaignMetadata>**](CampaignMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rewards_epoch_config_metadata

> Vec<models::EpochConfigs> get_rewards_epoch_config_metadata()
Gets the latest epoch configs for the campaigns

Returns the latest epoch configs for the campaigns

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::EpochConfigs>**](EpochConfigs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rewards_epoch_metadata

> Vec<models::EpochMetadata> get_rewards_epoch_metadata(campaign_name, epoch)
Gets the latest or next epoch for campaign.

Returns the latest or next epocht epoch for campaign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_name** | Option<**String**> | Specify the campaign name |  |
**epoch** | Option<**String**> | Specify the string \"next\" or \"latest\". |  |

### Return type

[**Vec<models::EpochMetadata>**](EpochMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rewards_interval_metadata

> Vec<models::IntervalMetadata> get_rewards_interval_metadata(interval)
Gets the interval metadata for provided parameters

Returns the interval metadata for provided parameters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval** | Option<**i32**> | Either specify an interval number or the string \"next\" or \"latest\". |  |

### Return type

[**Vec<models::IntervalMetadata>**](IntervalMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rewards_summary

> Vec<models::RewardsSummary> get_rewards_summary()
Get rewards information for all time rewards earned

Returns the all time rewards earned by users.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::RewardsSummary>**](RewardsSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

