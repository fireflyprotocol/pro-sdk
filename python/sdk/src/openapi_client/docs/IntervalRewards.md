# IntervalRewards


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | **str** | User address for the rewards earned data. | 
**status** | **str** |  | 
**blue_rewards_e9** | **str** | Total Blue token rewards earned in the interval (e9 format). | 
**sui_rewards_e9** | **str** | Total Sui token rewards earned in the interval (e9 format). | 
**wal_rewards_e9** | **str** | Total wal rewards earned in the interval (e9 format). | 
**interval_number** | **int** | Interval Id of the interval for the rewards earned data. | 

## Example

```python
from openapi_client.models.interval_rewards import IntervalRewards

# TODO update the JSON string below
json = "{}"
# create an instance of IntervalRewards from a JSON string
interval_rewards_instance = IntervalRewards.from_json(json)
# print the JSON string representation of the object
print(IntervalRewards.to_json())

# convert the object into a dict
interval_rewards_dict = interval_rewards_instance.to_dict()
# create an instance of IntervalRewards from a dict
interval_rewards_from_dict = IntervalRewards.from_dict(interval_rewards_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


