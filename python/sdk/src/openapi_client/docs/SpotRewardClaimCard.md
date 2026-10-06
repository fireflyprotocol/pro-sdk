# SpotRewardClaimCard


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | **str** |  | 
**pool_id** | **str** | On-chain pool object address. | 
**pool_name** | **str** |  | 
**claim_timestamp** | **str** | Claim time in epoch-ms, serialized as a string. | 
**tx_digest** | **str** |  | 
**rewards** | [**List[SpotRewardClaimEntry]**](SpotRewardClaimEntry.md) |  | 

## Example

```python
from openapi_client.models.spot_reward_claim_card import SpotRewardClaimCard

# TODO update the JSON string below
json = "{}"
# create an instance of SpotRewardClaimCard from a JSON string
spot_reward_claim_card_instance = SpotRewardClaimCard.from_json(json)
# print the JSON string representation of the object
print(SpotRewardClaimCard.to_json())

# convert the object into a dict
spot_reward_claim_card_dict = spot_reward_claim_card_instance.to_dict()
# create an instance of SpotRewardClaimCard from a dict
spot_reward_claim_card_from_dict = SpotRewardClaimCard.from_dict(spot_reward_claim_card_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


