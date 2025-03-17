# CreateOrderRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signed_fields** | [**CreateOrderRequestSignedFields**](CreateOrderRequestSignedFields.md) |  | 
**signature** | **str** | The signature of the request, encoded from the signedFields | 
**client_order_id** | **str** | The client-defined unique identifier of this order used for lookup. This should always be unique; however, the server will not gurantee this or impose any checks. | [optional] 
**type** | [**OrderType**](OrderType.md) |  | 
**reduce_only** | **bool** | Is this order to only reduce a position? Default false | 
**post_only** | **bool** | If set to TRUE, the order can only be a maker order | [optional] [default to False]
**time_in_force** | [**OrderTimeInForce**](OrderTimeInForce.md) | Omit or set to null for market orders; otherwise, choose a valid time-in-force value. GTT: Good Til Time  IOC: Immediate Or Cancel  FOK: Fill Or Kill  | [optional] 
**trigger_price_e9** | **str** | Trigger price in base e9 for stop orders. This should always be a number | [optional] 
**self_trade_prevention_type** | [**SelfTradePreventionType**](SelfTradePreventionType.md) |  | [optional] [default to SelfTradePreventionType.MAKER]

## Example

```python
from openapi_client.models.create_order_request import CreateOrderRequest

# TODO update the JSON string below
json = "{}"
# create an instance of CreateOrderRequest from a JSON string
create_order_request_instance = CreateOrderRequest.from_json(json)
# print the JSON string representation of the object
print(CreateOrderRequest.to_json())

# convert the object into a dict
create_order_request_dict = create_order_request_instance.to_dict()
# create an instance of CreateOrderRequest from a dict
create_order_request_from_dict = CreateOrderRequest.from_dict(create_order_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


