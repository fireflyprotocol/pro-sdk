# \RewardsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_affiliate_interval_overview**](RewardsApi.md#get_affiliate_interval_overview) | **GET** /v1/rewards/affiliate/intervalOverview | Get affiliate earnings overview by interval
[**get_affiliate_leader_dashboard**](RewardsApi.md#get_affiliate_leader_dashboard) | **GET** /v1/rewards/affiliate/leaderDashboard | Get affiliate rankings and earnings
[**get_affiliate_metadata**](RewardsApi.md#get_affiliate_metadata) | **GET** /v1/rewards/affiliate | Get affiliate metadata
[**get_affiliate_overview**](RewardsApi.md#get_affiliate_overview) | **GET** /v1/rewards/affiliate/overview | Get detailed affiliate earnings overview
[**get_affiliate_summary**](RewardsApi.md#get_affiliate_summary) | **GET** /v1/rewards/affiliate/summary | Get affiliate performance summary
[**get_campaign_rewards**](RewardsApi.md#get_campaign_rewards) | **GET** /v1/rewards/campaign | Get rewards information for a specific campaign
[**get_rewards**](RewardsApi.md#get_rewards) | **GET** /v1/rewards | Get rewards information for the intervals
[**get_rewards_campaign_metadata**](RewardsApi.md#get_rewards_campaign_metadata) | **GET** /v1/rewards/metadata/campaign | Get rewards metadata for the campaigns
[**get_rewards_epoch_config_metadata**](RewardsApi.md#get_rewards_epoch_config_metadata) | **GET** /v1/rewards/metadata/epoch/configs | Gets the latest epoch configs for the campaigns
[**get_rewards_epoch_metadata**](RewardsApi.md#get_rewards_epoch_metadata) | **GET** /v1/rewards/metadata/epoch | Gets the latest or next epoch for campaign.
[**get_rewards_interval_metadata**](RewardsApi.md#get_rewards_interval_metadata) | **GET** /v1/rewards/metadata/interval | Gets the interval metadata for provided parameters
[**get_rewards_summary**](RewardsApi.md#get_rewards_summary) | **GET** /v1/rewards/summary | Get rewards information for all time rewards earned
[**onboard_affiliate**](RewardsApi.md#onboard_affiliate) | **POST** /v1/rewards/affiliate/onboard | Submit affiliate onboarding application
[**onboard_referee**](RewardsApi.md#onboard_referee) | **POST** /v1/rewards/affiliate/onboard/referee | Onboard referee with a referral code
[**update_affiliate_fee_config**](RewardsApi.md#update_affiliate_fee_config) | **POST** /v1/rewards/affiliate/feeConfig | Update affiliate fee config



## get_affiliate_interval_overview

> models::GetAffiliateIntervalOverview200Response get_affiliate_interval_overview(user_address, page, limit)
Get affiliate earnings overview by interval

Returns detailed earnings breakdown for an affiliate by interval, ordered by interval number in descending order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_address** | **String** | The address of the user to get interval overview for | [required] |
**page** | Option<**u32**> | The page number to retrieve in a paginated response |  |[default to 1]
**limit** | Option<**u32**> | The page size for pagination |  |[default to 500]

### Return type

[**models::GetAffiliateIntervalOverview200Response**](getAffiliateIntervalOverview_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_affiliate_leader_dashboard

> models::GetAffiliateLeaderDashboard200Response get_affiliate_leader_dashboard(sort_by, sort_order, page, limit, name, user_address)
Get affiliate rankings and earnings

Returns rankings and earnings for affiliates, sorted by the specified category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort_by** | **String** | The category to sort rankings by | [required] |[default to perpsRank]
**sort_order** | Option<**String**> | The order to sort rankings by |  |
**page** | Option<**u32**> | The page number to retrieve in a paginated response |  |[default to 1]
**limit** | Option<**u32**> | The page size for pagination |  |[default to 500]
**name** | Option<**String**> | The name of the user to filter by |  |
**user_address** | Option<**String**> | The user address to filter by |  |

### Return type

[**models::GetAffiliateLeaderDashboard200Response**](getAffiliateLeaderDashboard_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_affiliate_metadata

> models::AffiliateMetadata get_affiliate_metadata()
Get affiliate metadata

Returns the affiliate metadata

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AffiliateMetadata**](AffiliateMetadata.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_affiliate_overview

> models::GetAffiliateOverview200Response get_affiliate_overview(page, limit, sort_by, sort_order, name, user_address)
Get detailed affiliate earnings overview

Returns detailed earnings breakdown for an affiliate users earnings (including perps, spot LP, lending), referral earnings, and total earnings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**u32**> | The page number to retrieve in a paginated response |  |[default to 1]
**limit** | Option<**u32**> | The page size for pagination |  |[default to 500]
**sort_by** | Option<**String**> | The category to sort earnings by |  |[default to totalEarnings]
**sort_order** | Option<**String**> | The order to sort earnings by |  |
**name** | Option<**String**> | The name of the user to filter by |  |
**user_address** | Option<**String**> | The user address to filter by |  |

### Return type

[**models::GetAffiliateOverview200Response**](getAffiliateOverview_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_affiliate_summary

> models::AffiliateSummary get_affiliate_summary()
Get affiliate performance summary

Returns performance summary for an affiliate including total referrals, earnings, and rankings

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AffiliateSummary**](AffiliateSummary.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_campaign_rewards

> Vec<models::UserCampaignRewards> get_campaign_rewards(campaign_name, epoch_number)
Get rewards information for a specific campaign

Returns the rewards earned by users for a specific campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_name** | **String** | Specify the campaign name | [required] |
**epoch_number** | Option<**i32**> | Optionally specify epoch number. |  |

### Return type

[**Vec<models::UserCampaignRewards>**](UserCampaignRewards.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

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

[bearerAuth](../README.md#bearerAuth)

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

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## onboard_affiliate

> models::AffiliateOnboardResponse onboard_affiliate(onboard_affiliate_request)
Submit affiliate onboarding application

Submit an application to become an affiliate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onboard_affiliate_request** | [**OnboardAffiliateRequest**](OnboardAffiliateRequest.md) |  | [required] |

### Return type

[**models::AffiliateOnboardResponse**](AffiliateOnboardResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## onboard_referee

> models::RefereeOnboardResponse onboard_referee(onboard_referee_request)
Onboard referee with a referral code

Onboard a referee with a referral code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onboard_referee_request** | [**OnboardRefereeRequest**](OnboardRefereeRequest.md) |  | [required] |

### Return type

[**models::RefereeOnboardResponse**](RefereeOnboardResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_affiliate_fee_config

> models::AffiliateMetadata update_affiliate_fee_config(update_affiliate_fee_config_request)
Update affiliate fee config

Update the fee config for an affiliate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_affiliate_fee_config_request** | [**UpdateAffiliateFeeConfigRequest**](UpdateAffiliateFeeConfigRequest.md) |  | [required] |

### Return type

[**models::AffiliateMetadata**](AffiliateMetadata.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

