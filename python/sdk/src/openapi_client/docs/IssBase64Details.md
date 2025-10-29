# IssBase64Details

Base64 encoded ISS details with index.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**value** | **str** | Base64 encoded ISS details value. | 
**index_mod4** | **int** | Index modulo 4 for the ISS details. | 

## Example

```python
from openapi_client.models.iss_base64_details import IssBase64Details

# TODO update the JSON string below
json = "{}"
# create an instance of IssBase64Details from a JSON string
iss_base64_details_instance = IssBase64Details.from_json(json)
# print the JSON string representation of the object
print(IssBase64Details.to_json())

# convert the object into a dict
iss_base64_details_dict = iss_base64_details_instance.to_dict()
# create an instance of IssBase64Details from a dict
iss_base64_details_from_dict = IssBase64Details.from_dict(iss_base64_details_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


