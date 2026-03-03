# KoraCampaignBonus


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bonus_type** | **str** | Type of bonus (e.g., VOLUME_BONUS, REFERRAL_BONUS) | [optional] 
**bonus_cc_reward** | **str** | CC reward amount for this bonus | [optional] 
**bonus_points** | **str** | Points earned for this bonus | [optional] 
**bonus_criteria** | **str** | Criteria that was met for earning this bonus | [optional] 

## Example

```python
from openapi_client.models.kora_campaign_bonus import KoraCampaignBonus

# TODO update the JSON string below
json = "{}"
# create an instance of KoraCampaignBonus from a JSON string
kora_campaign_bonus_instance = KoraCampaignBonus.from_json(json)
# print the JSON string representation of the object
print(KoraCampaignBonus.to_json())

# convert the object into a dict
kora_campaign_bonus_dict = kora_campaign_bonus_instance.to_dict()
# create an instance of KoraCampaignBonus from a dict
kora_campaign_bonus_from_dict = KoraCampaignBonus.from_dict(kora_campaign_bonus_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


