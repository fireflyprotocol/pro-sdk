# KoraCampaignRewards


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | **str** | User address for the rewards earned data | 
**campaign_name** | **str** | Name of the campaign | 
**epoch_number** | **int** | Epoch number for the rewards earned data | 
**interval_number** | **int** | Interval number for the rewards earned data | 
**symbol** | **str** | Market Symbol | [optional] 
**status** | **str** |  | 
**blue_rewards** | **str** | Total blue token rewards | [optional] 
**sui_rewards** | **str** | Total sui token rewards | [optional] 
**wal_rewards** | **str** | Total wal rewards | [optional] 
**cash_rewards** | **str** | Total cash rewards | [optional] 
**cc_rewards** | **str** | Total CC token rewards | [optional] 
**kora_points** | **str** | Total Kora points earned | [optional] 
**user_fee_paid** | **str** | Total user fee paid | [optional] 
**interval_start_date** | **int** | Time in milliseconds for interval start date | [optional] 
**interval_end_date** | **int** | Time in milliseconds for interval end date | [optional] 
**is_disbursed** | **bool** | Indicates if rewards have been disbursed | [optional] 
**txn_digest** | **str** | Transaction digest of the disbursement | [optional] 
**claim_status** | **str** | Status of the claim | [optional] 
**bonuses** | [**List[KoraCampaignBonus]**](KoraCampaignBonus.md) | List of bonuses attached to this reward entry | [optional] 

## Example

```python
from openapi_client.models.kora_campaign_rewards import KoraCampaignRewards

# TODO update the JSON string below
json = "{}"
# create an instance of KoraCampaignRewards from a JSON string
kora_campaign_rewards_instance = KoraCampaignRewards.from_json(json)
# print the JSON string representation of the object
print(KoraCampaignRewards.to_json())

# convert the object into a dict
kora_campaign_rewards_dict = kora_campaign_rewards_instance.to_dict()
# create an instance of KoraCampaignRewards from a dict
kora_campaign_rewards_from_dict = KoraCampaignRewards.from_dict(kora_campaign_rewards_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


