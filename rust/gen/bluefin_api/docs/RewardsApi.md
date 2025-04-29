# \RewardsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_rewards_campaign_get**](RewardsApi.md#v1_rewards_campaign_get) | **GET** /v1/rewards/campaign | Get rewards information for a specific campaign
[**v1_rewards_get**](RewardsApi.md#v1_rewards_get) | **GET** /v1/rewards | Get rewards information for the intervals
[**v1_rewards_metadata_campaign_get**](RewardsApi.md#v1_rewards_metadata_campaign_get) | **GET** /v1/rewards/metadata/campaign | Get rewards metadata for the campaigns
[**v1_rewards_metadata_epoch_configs_get**](RewardsApi.md#v1_rewards_metadata_epoch_configs_get) | **GET** /v1/rewards/metadata/epoch/configs | Gets the latest epoch configs for the campaigns
[**v1_rewards_metadata_epoch_get**](RewardsApi.md#v1_rewards_metadata_epoch_get) | **GET** /v1/rewards/metadata/epoch | Gets the latest or next epoch for campaign.
[**v1_rewards_metadata_interval_get**](RewardsApi.md#v1_rewards_metadata_interval_get) | **GET** /v1/rewards/metadata/interval | Gets the interval metadata for provided parameters
[**v1_rewards_summary_get**](RewardsApi.md#v1_rewards_summary_get) | **GET** /v1/rewards/summary | Get rewards information for all time rewards earned



## v1_rewards_campaign_get

> Vec<models::CampaignRewards> v1_rewards_campaign_get(campaign_name, epoch_number)
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


## v1_rewards_get

> Vec<models::IntervalRewards> v1_rewards_get(interval_number)
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


## v1_rewards_metadata_campaign_get

> Vec<models::CampaignMetadata> v1_rewards_metadata_campaign_get(campaign_name, status)
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


## v1_rewards_metadata_epoch_configs_get

> Vec<models::EpochConfigs> v1_rewards_metadata_epoch_configs_get()
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


## v1_rewards_metadata_epoch_get

> Vec<models::EpochMetadata> v1_rewards_metadata_epoch_get(campaign_name, epoch)
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


## v1_rewards_metadata_interval_get

> Vec<models::IntervalMetadata> v1_rewards_metadata_interval_get(interval)
Gets the interval metadata for provided parameters

Returns the interval metadata for provided parameters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval** | Option<[**V1RewardsMetadataIntervalGetIntervalParameter**](.md)> | Either specify an interval number or the string \"next\" or \"latest\". |  |

### Return type

[**Vec<models::IntervalMetadata>**](IntervalMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_rewards_summary_get

> Vec<models::RewardsSummary> v1_rewards_summary_get()
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

