# \RewardsApi

All URIs are relative to *https://api.sui-staging.bluefin.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_affiliate_interval_overview**](RewardsApi.md#get_affiliate_interval_overview) | **GET** /v1/rewards/affiliate/intervalOverview | /rewards/affiliate/intervalOverview
[**get_affiliate_leader_dashboard**](RewardsApi.md#get_affiliate_leader_dashboard) | **GET** /v1/rewards/affiliate/leaderDashboard | /rewards/affiliate/leaderDashboard
[**get_affiliate_metadata**](RewardsApi.md#get_affiliate_metadata) | **GET** /v1/rewards/affiliate | /rewards/affiliate
[**get_affiliate_overview**](RewardsApi.md#get_affiliate_overview) | **GET** /v1/rewards/affiliate/overview | /rewards/affiliate/overview
[**get_affiliate_summary**](RewardsApi.md#get_affiliate_summary) | **GET** /v1/rewards/affiliate/summary | /rewards/affiliate/summary
[**get_campaign_rewards**](RewardsApi.md#get_campaign_rewards) | **GET** /v1/rewards/campaign | /rewards/campaign
[**get_contract_config**](RewardsApi.md#get_contract_config) | **GET** /v1/rewards/contract/config | Get contract configurations
[**get_rewards**](RewardsApi.md#get_rewards) | **GET** /v1/rewards | /rewards
[**get_rewards_campaign_metadata**](RewardsApi.md#get_rewards_campaign_metadata) | **GET** /v1/rewards/metadata/campaign | /rewards/metadata/campaign
[**get_rewards_epoch_config_metadata**](RewardsApi.md#get_rewards_epoch_config_metadata) | **GET** /v1/rewards/metadata/epoch/configs | /rewards/metadata/epoch/configs
[**get_rewards_epoch_metadata**](RewardsApi.md#get_rewards_epoch_metadata) | **GET** /v1/rewards/metadata/epoch | /rewards/metadata/epoch
[**get_rewards_interval_metadata**](RewardsApi.md#get_rewards_interval_metadata) | **GET** /v1/rewards/metadata/interval | /rewards/metadata/interval
[**get_rewards_summary**](RewardsApi.md#get_rewards_summary) | **GET** /v1/rewards/summary | /rewards/summary
[**mark_as_claimed**](RewardsApi.md#mark_as_claimed) | **POST** /v1/rewards/claims/mark-claimed | /v1/rewards/claims/mark-claimed
[**onboard_affiliate**](RewardsApi.md#onboard_affiliate) | **POST** /v1/rewards/affiliate/onboard | /rewards/affiliate/onboard
[**onboard_referee**](RewardsApi.md#onboard_referee) | **POST** /v1/rewards/affiliate/onboard/referee | /rewards/affiliate/onboard/referee
[**update_affiliate_ember_refferal_share**](RewardsApi.md#update_affiliate_ember_refferal_share) | **POST** /v1/rewards/affiliate/emberRefferalShare | /rewards/affiliate/emberRefferalShare
[**update_affiliate_fee_config**](RewardsApi.md#update_affiliate_fee_config) | **POST** /v1/rewards/affiliate/feeConfig | /rewards/affiliate/feeConfig



## get_affiliate_interval_overview

> models::GetAffiliateIntervalOverview200Response get_affiliate_interval_overview(user_address, page, limit)
/rewards/affiliate/intervalOverview

Returns detailed earnings breakdown for an affiliate by interval, ordered by interval number in descending order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_address** | **String** | The address of the user to get interval overview for | [required] |
**page** | Option<**u32**> | The page number to retrieve in a paginated response |  |[default to 1]
**limit** | Option<**u32**> | The page size for pagination |  |[default to 500]

### Return type

[**models::GetAffiliateIntervalOverview200Response**](getAffiliateIntervalOverview_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_affiliate_leader_dashboard

> models::GetAffiliateLeaderDashboard200Response get_affiliate_leader_dashboard(sort_by, sort_order, page, limit, search)
/rewards/affiliate/leaderDashboard

Returns rankings and earnings for affiliates, sorted by the specified category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort_by** | Option<**String**> | The category to sort rankings by |  |[default to perpsRank]
**sort_order** | Option<**String**> | The order to sort rankings by |  |
**page** | Option<**u32**> | The page number to retrieve in a paginated response |  |[default to 1]
**limit** | Option<**u32**> | The page size for pagination |  |[default to 500]
**search** | Option<**String**> | The name/address of the user to filter by |  |

### Return type

[**models::GetAffiliateLeaderDashboard200Response**](getAffiliateLeaderDashboard_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_affiliate_metadata

> models::AffiliateMetadata get_affiliate_metadata(user_address)
/rewards/affiliate

Returns the affiliate metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_address** | **String** | Specify wallet address. | [required] |

### Return type

[**models::AffiliateMetadata**](AffiliateMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_affiliate_overview

> models::GetAffiliateOverview200Response get_affiliate_overview(user_address, page, limit, sort_by, sort_order, search, min_earnings_e9)
/rewards/affiliate/overview

Returns detailed earnings breakdown for an affiliate users earnings (including perps, spot LP, lending), referral earnings, and total earnings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_address** | **String** | Specify wallet address. | [required] |
**page** | Option<**u32**> | The page number to retrieve in a paginated response |  |[default to 1]
**limit** | Option<**u32**> | The page size for pagination |  |[default to 500]
**sort_by** | Option<**String**> | The category to sort earnings by |  |[default to totalEarnings]
**sort_order** | Option<**String**> | The order to sort earnings by |  |
**search** | Option<**String**> | The name/address of the user to filter by |  |
**min_earnings_e9** | Option<**String**> | The minimum earnings to filter by |  |[default to 0]

### Return type

[**models::GetAffiliateOverview200Response**](getAffiliateOverview_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_affiliate_summary

> models::AffiliateSummary get_affiliate_summary(user_address)
/rewards/affiliate/summary

Returns performance summary for an affiliate including total referrals, earnings, and rankings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_address** | **String** | Specify wallet address. | [required] |

### Return type

[**models::AffiliateSummary**](AffiliateSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_campaign_rewards

> Vec<models::UserCampaignRewards> get_campaign_rewards(campaign_name, user_address, epoch_number)
/rewards/campaign

Returns the rewards earned by users for a specific campaign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_name** | **String** | Specify the campaign name | [required] |
**user_address** | **String** | Specify wallet address. | [required] |
**epoch_number** | Option<**i32**> | Optionally specify epoch number. |  |

### Return type

[**Vec<models::UserCampaignRewards>**](UserCampaignRewards.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contract_config

> models::ContractConfig get_contract_config()
Get contract configurations

Returns the contract configuration metadata

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ContractConfig**](ContractConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rewards

> Vec<models::IntervalRewards> get_rewards(interval_number)
/rewards

Returns the rewards earned by users for the intervals.

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
/rewards/metadata/campaign

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

> models::EpochConfigsResponse get_rewards_epoch_config_metadata(interval_number)
/rewards/metadata/epoch/configs

Returns the latest epoch configs for the campaigns.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval_number** | Option<**i32**> | Specify the interval number |  |

### Return type

[**models::EpochConfigsResponse**](EpochConfigsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rewards_epoch_metadata

> Vec<models::EpochMetadata> get_rewards_epoch_metadata(campaign_name, epoch)
/rewards/metadata/epoch

Returns the latest or next epoch epoch for campaign.

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
/rewards/metadata/interval

Returns the interval metadata for provided parameters.

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
/rewards/summary

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


## mark_as_claimed

> models::MarkAsClaimedResponse mark_as_claimed(mark_as_claimed_request)
/v1/rewards/claims/mark-claimed

Mark user claims as claimed for the specified campaign name and interval number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mark_as_claimed_request** | [**MarkAsClaimedRequest**](MarkAsClaimedRequest.md) |  | [required] |

### Return type

[**models::MarkAsClaimedResponse**](MarkAsClaimedResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## onboard_affiliate

> models::AffiliateOnboardResponse onboard_affiliate(onboard_affiliate_request)
/rewards/affiliate/onboard

Submit an application to become an affiliate.

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
/rewards/affiliate/onboard/referee

Onboard a referee with a referral code.

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


## update_affiliate_ember_refferal_share

> models::AffiliateMetadata update_affiliate_ember_refferal_share(update_affiliate_ember_refferal_share_request)
/rewards/affiliate/emberRefferalShare

Update the ember refferal share for an affiliate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_affiliate_ember_refferal_share_request** | [**UpdateAffiliateEmberRefferalShareRequest**](UpdateAffiliateEmberRefferalShareRequest.md) |  | [required] |

### Return type

[**models::AffiliateMetadata**](AffiliateMetadata.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_affiliate_fee_config

> models::AffiliateMetadata update_affiliate_fee_config(update_affiliate_fee_config_request)
/rewards/affiliate/feeConfig

Update the fee config for an affiliate.

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

