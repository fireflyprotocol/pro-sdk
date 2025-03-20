# AssetConfig


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
from openapi_client.models.asset_config import AssetConfig

# TODO update the JSON string below
json = "{}"
# create an instance of AssetConfig from a JSON string
asset_config_instance = AssetConfig.from_json(json)
# print the JSON string representation of the object
print(AssetConfig.to_json())

# convert the object into a dict
asset_config_dict = asset_config_instance.to_dict()
# create an instance of AssetConfig from a dict
asset_config_from_dict = AssetConfig.from_dict(asset_config_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


