# RecentTradesUpdate


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **int** | The unique identifier of the trade. | 
**symbol** | **str** | The symbol of the market where the trade occurred. | 
**price_e9** | **str** | The trade price in scientific notation with 9 decimal places of precision. | 
**quantity_e9** | **str** | The quantity traded in scientific notation with 9 decimal places of precision. | 
**quote_quantity_e9** | **str** | The quote quantity traded in scientific notation with 9 decimal places of precision. | 
**side** | [**Side**](Side.md) |  | 
**updated_at_utc_millis** | **int** | The timestamp of the trade. | 

## Example

```python
from openapi_client.models.recent_trades_update import RecentTradesUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of RecentTradesUpdate from a JSON string
recent_trades_update_instance = RecentTradesUpdate.from_json(json)
# print the JSON string representation of the object
print(RecentTradesUpdate.to_json())

# convert the object into a dict
recent_trades_update_dict = recent_trades_update_instance.to_dict()
# create an instance of RecentTradesUpdate from a dict
recent_trades_update_from_dict = RecentTradesUpdate.from_dict(recent_trades_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


