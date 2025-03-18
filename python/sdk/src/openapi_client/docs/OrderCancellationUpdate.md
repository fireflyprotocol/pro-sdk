# OrderCancellationUpdate

Details of an order cancellation.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_hash** | **str** | The unique hash of the order. | 
**client_order_id** | **str** | The client-provided order ID. | [optional] 
**symbol** | **str** | The symbol of the market. | 
**account_address** | **str** | The address of the account. | 
**created_at_millis** | **int** | The timestamp of the order creation in milliseconds. | 
**cancellation_reason** | [**OrderCancelReason**](OrderCancelReason.md) |  | 
**failure_to_cancel_reason** | [**OrderCancellationFailureReason**](OrderCancellationFailureReason.md) |  | [optional] 
**remaining_quantity_e9** | **str** | The remaining quantity of the order. | 

## Example

```python
from openapi_client.models.order_cancellation_update import OrderCancellationUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of OrderCancellationUpdate from a JSON string
order_cancellation_update_instance = OrderCancellationUpdate.from_json(json)
# print the JSON string representation of the object
print(OrderCancellationUpdate.to_json())

# convert the object into a dict
order_cancellation_update_dict = order_cancellation_update_instance.to_dict()
# create an instance of OrderCancellationUpdate from a dict
order_cancellation_update_from_dict = OrderCancellationUpdate.from_dict(order_cancellation_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


