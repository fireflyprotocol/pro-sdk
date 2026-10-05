# SpotRewardClaimEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reward_type** | **str** | Reward/fee token symbol; a fee and reward of the same symbol are merged. | 
**rewarded_amount** | **str** | Claimed amount in the token&#39;s own units (decimal string, e.g. \&quot;0.61172\&quot;). | 

## Example

```python
from openapi_client.models.spot_reward_claim_entry import SpotRewardClaimEntry

# TODO update the JSON string below
json = "{}"
# create an instance of SpotRewardClaimEntry from a JSON string
spot_reward_claim_entry_instance = SpotRewardClaimEntry.from_json(json)
# print the JSON string representation of the object
print(SpotRewardClaimEntry.to_json())

# convert the object into a dict
spot_reward_claim_entry_dict = spot_reward_claim_entry_instance.to_dict()
# create an instance of SpotRewardClaimEntry from a dict
spot_reward_claim_entry_from_dict = SpotRewardClaimEntry.from_dict(spot_reward_claim_entry_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


