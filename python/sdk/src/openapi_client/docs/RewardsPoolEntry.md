# RewardsPoolEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**coin** | **str** |  | [optional] 

## Example

```python
from openapi_client.models.rewards_pool_entry import RewardsPoolEntry

# TODO update the JSON string below
json = "{}"
# create an instance of RewardsPoolEntry from a JSON string
rewards_pool_entry_instance = RewardsPoolEntry.from_json(json)
# print the JSON string representation of the object
print(RewardsPoolEntry.to_json())

# convert the object into a dict
rewards_pool_entry_dict = rewards_pool_entry_instance.to_dict()
# create an instance of RewardsPoolEntry from a dict
rewards_pool_entry_from_dict = RewardsPoolEntry.from_dict(rewards_pool_entry_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


