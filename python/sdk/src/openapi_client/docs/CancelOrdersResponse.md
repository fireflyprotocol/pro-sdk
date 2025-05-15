# CancelOrdersResponse

Response to a request to cancel orders.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_hashes** | **List[str]** | The order hashes of the cancelled orders. | 

## Example

```python
from openapi_client.models.cancel_orders_response import CancelOrdersResponse

# TODO update the JSON string below
json = "{}"
# create an instance of CancelOrdersResponse from a JSON string
cancel_orders_response_instance = CancelOrdersResponse.from_json(json)
# print the JSON string representation of the object
print(CancelOrdersResponse.to_json())

# convert the object into a dict
cancel_orders_response_dict = cancel_orders_response_instance.to_dict()
# create an instance of CancelOrdersResponse from a dict
cancel_orders_response_from_dict = CancelOrdersResponse.from_dict(cancel_orders_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


