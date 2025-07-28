# EpochConfigsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_interval_number** | **int** | The maximum interval number available. | 
**interval_number** | **int** | The current interval number being queried. | 
**data** | [**List[EpochConfigs]**](EpochConfigs.md) | List of epoch configs for different campaigns. | 

## Example

```python
from openapi_client.models.epoch_configs_response import EpochConfigsResponse

# TODO update the JSON string below
json = "{}"
# create an instance of EpochConfigsResponse from a JSON string
epoch_configs_response_instance = EpochConfigsResponse.from_json(json)
# print the JSON string representation of the object
print(EpochConfigsResponse.to_json())

# convert the object into a dict
epoch_configs_response_dict = epoch_configs_response_instance.to_dict()
# create an instance of EpochConfigsResponse from a dict
epoch_configs_response_from_dict = EpochConfigsResponse.from_dict(epoch_configs_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


