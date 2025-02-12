# MarkPriceUpdate


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | The symbol of the market. | 
**price_e9** | **str** | The price in scientific notation with 9 decimal places of precision. | 
**source** | **str** |  | 
**updated_at_utc_millis** | **int** | The timestamp of the price update. | 

## Example

```python
from openapi_client.models.mark_price_update import MarkPriceUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of MarkPriceUpdate from a JSON string
mark_price_update_instance = MarkPriceUpdate.from_json(json)
# print the JSON string representation of the object
print(MarkPriceUpdate.to_json())

# convert the object into a dict
mark_price_update_dict = mark_price_update_instance.to_dict()
# create an instance of MarkPriceUpdate from a dict
mark_price_update_from_dict = MarkPriceUpdate.from_dict(mark_price_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


