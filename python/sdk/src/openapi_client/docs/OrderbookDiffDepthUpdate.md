# OrderbookDiffDepthUpdate


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updated_at_millis** | **int** | The timestamp of the orderbook update. | 
**symbol** | **str** | The symbol of the market for the orderbook update. | 
**bids_e9** | **List[List[str]]** |  | 
**asks_e9** | **List[List[str]]** |  | 
**first_update_id** | **int** | The ID of the first update in this batch. | 
**last_update_id** | **int** | The ID of the last update in this batch. | 

## Example

```python
from openapi_client.models.orderbook_diff_depth_update import OrderbookDiffDepthUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of OrderbookDiffDepthUpdate from a JSON string
orderbook_diff_depth_update_instance = OrderbookDiffDepthUpdate.from_json(json)
# print the JSON string representation of the object
print(OrderbookDiffDepthUpdate.to_json())

# convert the object into a dict
orderbook_diff_depth_update_dict = orderbook_diff_depth_update_instance.to_dict()
# create an instance of OrderbookDiffDepthUpdate from a dict
orderbook_diff_depth_update_from_dict = OrderbookDiffDepthUpdate.from_dict(orderbook_diff_depth_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


