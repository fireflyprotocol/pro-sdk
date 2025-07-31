# EpochConfigs


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**campaign_name** | **str** | The name of the campaign. | 
**epoch_duration** | **int** | Duration of the epoch in seconds. | 
**sui_rewards_allocation_e9** | **str** | Allocation of Sui token rewards in the epoch (e9 format). | 
**blue_rewards_allocation_e9** | **str** | Allocation of Blue token rewards in the epoch (e9 format). | 
**wal_rewards_allocation_e9** | **str** | Allocation of wal token rewards in the epoch (e9 format) | 
**interval_number** | **int** | Interval number for the epoch. | 
**epoch_number** | **int** | Epoch number for the epoch. | 
**config** | **Dict[str, object]** | Object to add custom configurations for campaigns. | 

## Example

```python
from openapi_client.models.epoch_configs import EpochConfigs

# TODO update the JSON string below
json = "{}"
# create an instance of EpochConfigs from a JSON string
epoch_configs_instance = EpochConfigs.from_json(json)
# print the JSON string representation of the object
print(EpochConfigs.to_json())

# convert the object into a dict
epoch_configs_dict = epoch_configs_instance.to_dict()
# create an instance of EpochConfigs from a dict
epoch_configs_from_dict = EpochConfigs.from_dict(epoch_configs_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


