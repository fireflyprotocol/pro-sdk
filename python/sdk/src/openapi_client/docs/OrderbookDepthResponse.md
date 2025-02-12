# OrderbookDepthResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | Market symbol. | 
**last_update_id** | **int** | Count indicating the number of changes in orderbook state. | 
**last_updated_at_utc_millis** | **int** | Timestamp at which the last change in orderbook state took place, in milliseconds. | 
**response_sent_at_utc_millis** | **int** | The time at which the orderbook server sent the response, in milliseconds. | 
**best_bid_price_e9** | **str** | The best bid price on orderbook at the moment (e9 format). | 
**best_bid_quantity_e9** | **str** | The best bid quantity on orderbook at the moment (e9 format). | 
**best_ask_price_e9** | **str** | The best ask price on orderbook at the moment (e9 format). | 
**best_ask_quantity_e9** | **str** | The best ask quantity on orderbook at the moment (e9 format). | 
**bids_e9** | **List[List[str]]** | Bids to be filled. Index 0 is price, index 1 is quantity at price bin. Prices are in e9 format. | 
**asks_e9** | **List[List[str]]** | Asks to be filled. Index 0 is price, index 1 is quantity at price bin. Prices are in e9 format. | 

## Example

```python
from openapi_client.models.orderbook_depth_response import OrderbookDepthResponse

# TODO update the JSON string below
json = "{}"
# create an instance of OrderbookDepthResponse from a JSON string
orderbook_depth_response_instance = OrderbookDepthResponse.from_json(json)
# print the JSON string representation of the object
print(OrderbookDepthResponse.to_json())

# convert the object into a dict
orderbook_depth_response_dict = orderbook_depth_response_instance.to_dict()
# create an instance of OrderbookDepthResponse from a dict
orderbook_depth_response_from_dict = OrderbookDepthResponse.from_dict(orderbook_depth_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


