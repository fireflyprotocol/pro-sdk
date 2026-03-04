# KoraRewardsSummary


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | **str** | User address for the rewards earned data | 
**blue_rewards** | **str** | Total Blue token rewards earned | [optional] 
**sui_rewards** | **str** | Total Sui token rewards earned | [optional] 
**wal_rewards** | **str** | Total wal rewards earned | [optional] 
**cc_rewards** | **str** | Total CC token rewards earned across all Kora campaigns | [optional] 
**kora_points** | **str** | Total Kora points earned across all Kora campaigns | [optional] 

## Example

```python
from openapi_client.models.kora_rewards_summary import KoraRewardsSummary

# TODO update the JSON string below
json = "{}"
# create an instance of KoraRewardsSummary from a JSON string
kora_rewards_summary_instance = KoraRewardsSummary.from_json(json)
# print the JSON string representation of the object
print(KoraRewardsSummary.to_json())

# convert the object into a dict
kora_rewards_summary_dict = kora_rewards_summary_instance.to_dict()
# create an instance of KoraRewardsSummary from a dict
kora_rewards_summary_from_dict = KoraRewardsSummary.from_dict(kora_rewards_summary_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


