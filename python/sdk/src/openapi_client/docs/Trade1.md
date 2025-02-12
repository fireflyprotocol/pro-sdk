# Trade1


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **int** | Trade ID | 
**symbol** | **str** | Market symbol. | 
**price_e9** | **str** | Trade price (e9 format). | 
**quantity_e9** | **str** | Trade quantity (e9 format). | 
**quote_quantity_e9** | **str** | Trade quote quantity (e9 format). | 
**side** | [**TradeSide**](TradeSide.md) |  | 
**time_at_utc_millis** | **int** | Trade timestamp. | 

## Example

```python
from openapi_client.models.trade1 import Trade1

# TODO update the JSON string below
json = "{}"
# create an instance of Trade1 from a JSON string
trade1_instance = Trade1.from_json(json)
# print the JSON string representation of the object
print(Trade1.to_json())

# convert the object into a dict
trade1_dict = trade1_instance.to_dict()
# create an instance of Trade1 from a dict
trade1_from_dict = Trade1.from_dict(trade1_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


