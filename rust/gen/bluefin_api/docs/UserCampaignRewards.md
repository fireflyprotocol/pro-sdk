# UserCampaignRewards

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | **String** | User address for the rewards earned data. | 
**campaign_name** | **String** | Name of the campaign. | 
**epoch_number** | **i32** | Epoch number for the rewards earned data. | 
**interval_number** | **i32** | Interval number for the rewards earned data. | 
**symbol** | **String** | Market Symbol. | 
**status** | **String** |  | 
**blue_rewards_e9** | **String** | Total blue-perp token rewards earned in the epoch (e9 format). | 
**sui_rewards_e9** | **String** | Total sui-perp token rewards earned in the epoch (e9 format). | 
**wal_rewards_e9** | **String** | Total wal-perp rewards earned in the epoch (e9 format). | 
**cash_rewards_e9** | **String** | Total cash rewards earned in the epoch (e9 format). | 
**user_fee_paid_e9** | **String** | Total user fee paid in the epoch (e9 format). | 
**interval_start_date** | **i32** | Time in milliseconds for interval start date. | 
**interval_end_date** | **i32** | Time in milliseconds for interval end date. | 
**is_disbursed** | **bool** | Indicates if the rewards have been disbursed. | 
**txn_digest** | **String** | Transaction digest of the disbursement. | 
**claim_signature** | Option<[**Vec<models::ClaimSignatureItem>**](ClaimSignatureItem.md)> | Array of claim signatures for different reward types. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


