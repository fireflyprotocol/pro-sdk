# CancelOrdersRequest

Cancelling Orders for a specific symbol. If order hashes are not specified, all orders are canceled for this symbol

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | The symbol of the perpetual for which to cancel orders. | 
**order_hashes** | **List[str]** | List of order hashes of the orders to be cancelled. All orders must belong to accountAddress. Max 10 order hashes | [optional] 

## Example

```python
from openapi_client.models.cancel_orders_request import CancelOrdersRequest

# TODO update the JSON string below
json = "{}"
# create an instance of CancelOrdersRequest from a JSON string
cancel_orders_request_instance = CancelOrdersRequest.from_json(json)
# print the JSON string representation of the object
print(CancelOrdersRequest.to_json())

# convert the object into a dict
cancel_orders_request_dict = cancel_orders_request_instance.to_dict()
# create an instance of CancelOrdersRequest from a dict
cancel_orders_request_from_dict = CancelOrdersRequest.from_dict(cancel_orders_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


