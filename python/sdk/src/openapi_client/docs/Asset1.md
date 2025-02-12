# Asset1


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_type** | **str** | The bank address of the asset. | 
**symbol** | **str** | Asset symbol. | 
**decimals** | **int** | Default precision for rendering this asset. | 
**weight** | **str** | Weight applied to asset to use as margin in Multi-Assets mode. | 
**margin_available** | **bool** | Indicates if the asset can be used as margin in Multi-Assets mode. | 

## Example

```python
from openapi_client.models.asset1 import Asset1

# TODO update the JSON string below
json = "{}"
# create an instance of Asset1 from a JSON string
asset1_instance = Asset1.from_json(json)
# print the JSON string representation of the object
print(Asset1.to_json())

# convert the object into a dict
asset1_dict = asset1_instance.to_dict()
# create an instance of Asset1 from a dict
asset1_from_dict = Asset1.from_dict(asset1_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


