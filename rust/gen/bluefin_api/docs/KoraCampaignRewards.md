# KoraCampaignRewards

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | **String** | User address for the rewards earned data | 
**campaign_name** | **String** | Name of the campaign | 
**epoch_number** | **i32** | Epoch number for the rewards earned data | 
**interval_number** | **i32** | Interval number for the rewards earned data | 
**symbol** | Option<**String**> | Market Symbol | [optional]
**status** | **String** |  | 
**blue_rewards** | Option<**String**> | Total blue token rewards | [optional]
**sui_rewards** | Option<**String**> | Total sui token rewards | [optional]
**wal_rewards** | Option<**String**> | Total wal rewards | [optional]
**cash_rewards** | Option<**String**> | Total cash rewards | [optional]
**cc_rewards** | Option<**String**> | Total CC token rewards | [optional]
**kora_points** | Option<**String**> | Total Kora points earned | [optional]
**user_fee_paid** | Option<**String**> | Total user fee paid | [optional]
**interval_start_date** | Option<**i32**> | Time in milliseconds for interval start date | [optional]
**interval_end_date** | Option<**i32**> | Time in milliseconds for interval end date | [optional]
**is_disbursed** | Option<**bool**> | Indicates if rewards have been disbursed | [optional]
**txn_digest** | Option<**String**> | Transaction digest of the disbursement | [optional]
**claim_status** | Option<**String**> | Status of the claim | [optional]
**bonuses** | Option<[**Vec<models::KoraCampaignBonus>**](KoraCampaignBonus.md)> | List of bonuses attached to this reward entry | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


