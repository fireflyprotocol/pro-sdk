# CreateOrderRequestTwapConfig

Configuration for Time-Weighted Average Price orders

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sub_orders_count** | **str** | Number of sub-orders to split the total quantity into | [optional] 
**running_time_in_minutes** | **str** | Total time in minutes over which to execute the TWAP order | [optional] 

## Example

```python
from openapi_client.models.create_order_request_twap_config import CreateOrderRequestTwapConfig

# TODO update the JSON string below
json = "{}"
# create an instance of CreateOrderRequestTwapConfig from a JSON string
create_order_request_twap_config_instance = CreateOrderRequestTwapConfig.from_json(json)
# print the JSON string representation of the object
print(CreateOrderRequestTwapConfig.to_json())

# convert the object into a dict
create_order_request_twap_config_dict = create_order_request_twap_config_instance.to_dict()
# create an instance of CreateOrderRequestTwapConfig from a dict
create_order_request_twap_config_from_dict = CreateOrderRequestTwapConfig.from_dict(create_order_request_twap_config_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


