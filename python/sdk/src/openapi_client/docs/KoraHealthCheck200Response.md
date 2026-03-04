# KoraHealthCheck200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **str** |  | [optional] 
**timestamp** | **int** |  | [optional] 

## Example

```python
from openapi_client.models.kora_health_check200_response import KoraHealthCheck200Response

# TODO update the JSON string below
json = "{}"
# create an instance of KoraHealthCheck200Response from a JSON string
kora_health_check200_response_instance = KoraHealthCheck200Response.from_json(json)
# print the JSON string representation of the object
print(KoraHealthCheck200Response.to_json())

# convert the object into a dict
kora_health_check200_response_dict = kora_health_check200_response_instance.to_dict()
# create an instance of KoraHealthCheck200Response from a dict
kora_health_check200_response_from_dict = KoraHealthCheck200Response.from_dict(kora_health_check200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


