# KoraEpochConfigResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_interval_number** | **int** | The maximum interval number available | 
**interval_number** | **int** | The current interval number being queried | 
**data** | [**List[KoraEpochConfig]**](KoraEpochConfig.md) |  | 

## Example

```python
from openapi_client.models.kora_epoch_config_response import KoraEpochConfigResponse

# TODO update the JSON string below
json = "{}"
# create an instance of KoraEpochConfigResponse from a JSON string
kora_epoch_config_response_instance = KoraEpochConfigResponse.from_json(json)
# print the JSON string representation of the object
print(KoraEpochConfigResponse.to_json())

# convert the object into a dict
kora_epoch_config_response_dict = kora_epoch_config_response_instance.to_dict()
# create an instance of KoraEpochConfigResponse from a dict
kora_epoch_config_response_from_dict = KoraEpochConfigResponse.from_dict(kora_epoch_config_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


