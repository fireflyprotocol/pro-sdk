# OrderbookPartialDepthUpdate


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updated_at_millis** | **int** | The timestamp of the partial depth update. | 
**symbol** | **str** | The symbol of the market for the partial depth update. | 
**bids_e9** | **List[List[str]]** |  | 
**asks_e9** | **List[List[str]]** |  | 
**orderbook_update_id** | **int** | The unique identifier for the orderbook update. | 
**depth_level** | **str** | The depth level of the orderbook snapshot. | 

## Example

```python
from openapi_client.models.orderbook_partial_depth_update import OrderbookPartialDepthUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of OrderbookPartialDepthUpdate from a JSON string
orderbook_partial_depth_update_instance = OrderbookPartialDepthUpdate.from_json(json)
# print the JSON string representation of the object
print(OrderbookPartialDepthUpdate.to_json())

# convert the object into a dict
orderbook_partial_depth_update_dict = orderbook_partial_depth_update_instance.to_dict()
# create an instance of OrderbookPartialDepthUpdate from a dict
orderbook_partial_depth_update_from_dict = OrderbookPartialDepthUpdate.from_dict(orderbook_partial_depth_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


