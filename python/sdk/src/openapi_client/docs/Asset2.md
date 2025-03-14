# Asset2

Details about an asset in the account.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | The symbol of the asset. | 
**quantity_e9** | **str** | The quantity of the asset. | 
**effective_balance_e9** | **str** | The effective balance of the asset. | 
**max_withdraw_quantity_e9** | **str** | The maximum quantity that can be withdrawn. | 
**updated_at_millis** | **int** | The timestamp of the last update in milliseconds. | 

## Example

```python
from openapi_client.models.asset2 import Asset2

# TODO update the JSON string below
json = "{}"
# create an instance of Asset2 from a JSON string
asset2_instance = Asset2.from_json(json)
# print the JSON string representation of the object
print(Asset2.to_json())

# convert the object into a dict
asset2_dict = asset2_instance.to_dict()
# create an instance of Asset2 from a dict
asset2_from_dict = Asset2.from_dict(asset2_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


