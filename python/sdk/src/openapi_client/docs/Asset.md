# Asset


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | On-chain address of the asset bank on the blockchain. | 
**quantity_e9** | **str** | Asset quantity (e9 format). | 
**effective_balance_e9** | **str** | Value of this asset balance based on current market price and asset weight (e9 format). | 
**max_withdraw_quantity_e9** | **str** | Maximum quantity for transfer out (e9 format). | 
**last_updated_at_millis** | **int** | Last update time in milliseconds since Unix epoch. | 

## Example

```python
from openapi_client.models.asset import Asset

# TODO update the JSON string below
json = "{}"
# create an instance of Asset from a JSON string
asset_instance = Asset.from_json(json)
# print the JSON string representation of the object
print(Asset.to_json())

# convert the object into a dict
asset_dict = asset_instance.to_dict()
# create an instance of Asset from a dict
asset_from_dict = Asset.from_dict(asset_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


