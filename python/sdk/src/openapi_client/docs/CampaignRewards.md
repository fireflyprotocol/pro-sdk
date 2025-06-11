# CampaignRewards


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rewards** | [**UserCampaignRewards**](UserCampaignRewards.md) |  | [optional] 

## Example

```python
from openapi_client.models.campaign_rewards import CampaignRewards

# TODO update the JSON string below
json = "{}"
# create an instance of CampaignRewards from a JSON string
campaign_rewards_instance = CampaignRewards.from_json(json)
# print the JSON string representation of the object
print(CampaignRewards.to_json())

# convert the object into a dict
campaign_rewards_dict = campaign_rewards_instance.to_dict()
# create an instance of CampaignRewards from a dict
campaign_rewards_from_dict = CampaignRewards.from_dict(campaign_rewards_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


