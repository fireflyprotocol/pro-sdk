# RewardsSummary


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | **str** | User address for the rewards earned data. | 
**blue_rewards_e9** | **str** | Total Blue token rewards earned by the user (e9 format). | 
**sui_rewards_e9** | **str** | Total Sui token rewards earned by the user (e9 format). | 
**wal_rewards_e9** | **str** | Total wal rewards earned by the user (e9 format). | 

## Example

```python
from openapi_client.models.rewards_summary import RewardsSummary

# TODO update the JSON string below
json = "{}"
# create an instance of RewardsSummary from a JSON string
rewards_summary_instance = RewardsSummary.from_json(json)
# print the JSON string representation of the object
print(RewardsSummary.to_json())

# convert the object into a dict
rewards_summary_dict = rewards_summary_instance.to_dict()
# create an instance of RewardsSummary from a dict
rewards_summary_from_dict = RewardsSummary.from_dict(rewards_summary_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


