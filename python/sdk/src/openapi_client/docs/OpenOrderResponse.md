# OpenOrderResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_hash** | **str** | The Order Hash, which is the default way to uniquely identify an order in the system | 
**client_order_id** | **str** | The Client Order ID, which is used a unique identifier for an order, provided by the client, in case of proprietary order management systems | [optional] 
**symbol** | **str** | The market symbol | 
**account_address** | **str** | The account address of the order. May be an account user is authorized for. | 
**price_e9** | **str** | The price in base e9 of the asset to be traded. Should always be a number | 
**quantity_e9** | **str** | The quantity in base e9 of the asset to be traded. Should always be a number | 
**side** | [**OrderSide**](OrderSide.md) |  | 
**leverage_e9** | **str** | The leverage in base e9  of the order to be traded. Should always be a number | 
**is_isolated** | **bool** | Is this order isolated or cross margin. Note market must be set to the same mode. | [default to False]
**salt** | **str** | The random generated SALT. Should always be a number | 
**expires_at_millis** | **int** | Unix timestamp in millis at which order will expire. Defaults to 1 month for LIMIT orders if not provided | 
**signed_at_millis** | **int** | The timestamp in millis at which the request was signed | 
**type** | [**OrderType**](OrderType.md) |  | 
**reduce_only** | **bool** | Is this order to only reduce a position? Default false | [default to False]
**post_only** | **bool** | If set to TRUE, the order can only be a maker order | [default to False]
**time_in_force** | [**OrderTimeInForce**](OrderTimeInForce.md) |  | [default to OrderTimeInForce.GTT]
**trigger_price_e9** | **str** | Trigger price in base e9 for stop orders. This should always be a number | [optional] 
**filled_quantity_e9** | **str** | The quantity in base e9 of the asset currently filled. This should always be a number | 
**status** | [**OrderStatus**](OrderStatus.md) |  | 
**self_trade_prevention_type** | [**SelfTradePreventionType**](SelfTradePreventionType.md) |  | [default to SelfTradePreventionType.MAKER]
**order_time_at_millis** | **int** | The timestamp in millis when the order was opened | 
**updated_at_millis** | **int** | The timestamp in millis that this order was last updated (including status updates) | 

## Example

```python
from openapi_client.models.open_order_response import OpenOrderResponse

# TODO update the JSON string below
json = "{}"
# create an instance of OpenOrderResponse from a JSON string
open_order_response_instance = OpenOrderResponse.from_json(json)
# print the JSON string representation of the object
print(OpenOrderResponse.to_json())

# convert the object into a dict
open_order_response_dict = open_order_response_instance.to_dict()
# create an instance of OpenOrderResponse from a dict
open_order_response_from_dict = OpenOrderResponse.from_dict(open_order_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


