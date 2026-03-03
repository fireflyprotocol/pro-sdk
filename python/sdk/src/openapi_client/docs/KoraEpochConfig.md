# KoraEpochConfig


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**campaign_name** | **str** | The name of the campaign | 
**epoch_duration** | **int** | Duration of the epoch in hours | 
**sui_rewards_allocation** | **str** | Allocation of Sui rewards for this epoch | [optional] 
**blue_rewards_allocation** | **str** | Allocation of Blue rewards for this epoch | [optional] 
**wal_rewards_allocation** | **str** | Allocation of Wal rewards for this epoch | [optional] 
**cc_rewards_allocation** | **str** | Allocation of CC token rewards for this epoch | [optional] 
**kora_points_rewards_allocation** | **str** | Allocation of Kora points rewards for this epoch | [optional] 
**interval_number** | **int** | Interval number for the epoch | 
**epoch_number** | **int** | Epoch number | 
**config** | **Dict[str, object]** | Additional campaign-specific configurations | [optional] 

## Example

```python
from openapi_client.models.kora_epoch_config import KoraEpochConfig

# TODO update the JSON string below
json = "{}"
# create an instance of KoraEpochConfig from a JSON string
kora_epoch_config_instance = KoraEpochConfig.from_json(json)
# print the JSON string representation of the object
print(KoraEpochConfig.to_json())

# convert the object into a dict
kora_epoch_config_dict = kora_epoch_config_instance.to_dict()
# create an instance of KoraEpochConfig from a dict
kora_epoch_config_from_dict = KoraEpochConfig.from_dict(kora_epoch_config_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


