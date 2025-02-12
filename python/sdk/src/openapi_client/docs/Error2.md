# Error2


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error_code** | **str** | A code representing the type of error. | [optional] 
**message** | **str** | A human-readable message describing the error. | 

## Example

```python
from openapi_client.models.error2 import Error2

# TODO update the JSON string below
json = "{}"
# create an instance of Error2 from a JSON string
error2_instance = Error2.from_json(json)
# print the JSON string representation of the object
print(Error2.to_json())

# convert the object into a dict
error2_dict = error2_instance.to_dict()
# create an instance of Error2 from a dict
error2_from_dict = Error2.from_dict(error2_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


