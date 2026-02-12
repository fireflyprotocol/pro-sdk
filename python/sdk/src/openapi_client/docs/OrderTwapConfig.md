# OrderTwapConfig

Configuration for Time-Weighted Average Price orders

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sub_orders_count** | **str** | Number of sub-orders to split the total quantity into | [optional] 
**running_time_in_minutes** | **str** | Total time in minutes over which to execute the TWAP order | [optional] 

## Example

```python
from openapi_client.models.order_twap_config import OrderTwapConfig

# TODO update the JSON string below
json = "{}"
# create an instance of OrderTwapConfig from a JSON string
order_twap_config_instance = OrderTwapConfig.from_json(json)
# print the JSON string representation of the object
print(OrderTwapConfig.to_json())

# convert the object into a dict
order_twap_config_dict = order_twap_config_instance.to_dict()
# create an instance of OrderTwapConfig from a dict
order_twap_config_from_dict = OrderTwapConfig.from_dict(order_twap_config_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


