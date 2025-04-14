# Operators


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin** | **str** | Admin operator address | 
**operator** | **str** | General operator address; AKA Guardian | 
**sequencer** | **str** | Sequencer operator address | 
**funding** | **str** | Funding operator address | 
**fee** | **str** | Fee operator address | 

## Example

```python
from openapi_client.models.operators import Operators

# TODO update the JSON string below
json = "{}"
# create an instance of Operators from a JSON string
operators_instance = Operators.from_json(json)
# print the JSON string representation of the object
print(Operators.to_json())

# convert the object into a dict
operators_dict = operators_instance.to_dict()
# create an instance of Operators from a dict
operators_from_dict = Operators.from_dict(operators_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


