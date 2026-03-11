# OpenIDConfigurationResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issuer** | **str** |  | 
**authorization_endpoint** | **str** |  | 
**jwks_uri** | **str** |  | 
**token_endpoint** | **str** |  | 
**response_types_supported** | **List[str]** |  | 
**id_token_signing_alg_values_supported** | **List[str]** |  | 
**subject_types_supported** | **List[str]** |  | 
**scopes_supported** | **List[str]** |  | 

## Example

```python
from openapi_client.models.open_id_configuration_response import OpenIDConfigurationResponse

# TODO update the JSON string below
json = "{}"
# create an instance of OpenIDConfigurationResponse from a JSON string
open_id_configuration_response_instance = OpenIDConfigurationResponse.from_json(json)
# print the JSON string representation of the object
print(OpenIDConfigurationResponse.to_json())

# convert the object into a dict
open_id_configuration_response_dict = open_id_configuration_response_instance.to_dict()
# create an instance of OpenIDConfigurationResponse from a dict
open_id_configuration_response_from_dict = OpenIDConfigurationResponse.from_dict(open_id_configuration_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


