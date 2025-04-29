# UserCampaignRewards


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | **str** | User address for the rewards earned data. | 
**campaign_name** | **str** | Name of the campaign. | 
**epoch_number** | **int** | Epoch number for the rewards earned data. | 
**interval_number** | **int** | Interval number for the rewards earned data. | 
**market_address** | **str** | Market MarketAddress. | 
**status** | **str** |  | 
**blue_rewards** | **str** | Total Blue token rewards earned in the epoch (e9 format). | 
**sui_rewards** | **str** | Total Sui token rewards earned in the epoch (e9 format). | 
**cash_rewards** | **str** | Total cash rewards earned in the epoch (e9 format). | 

## Example

```python
from openapi_client.models.user_campaign_rewards import UserCampaignRewards

# TODO update the JSON string below
json = "{}"
# create an instance of UserCampaignRewards from a JSON string
user_campaign_rewards_instance = UserCampaignRewards.from_json(json)
# print the JSON string representation of the object
print(UserCampaignRewards.to_json())

# convert the object into a dict
user_campaign_rewards_dict = user_campaign_rewards_instance.to_dict()
# create an instance of UserCampaignRewards from a dict
user_campaign_rewards_from_dict = UserCampaignRewards.from_dict(user_campaign_rewards_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


