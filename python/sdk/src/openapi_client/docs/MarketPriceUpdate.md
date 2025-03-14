# MarketPriceUpdate


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | The symbol of the market. | 
**price_e9** | **str** | The price in scientific notation with 9 decimal places of precision. | 
**source** | **str** |  | 
**updated_at_millis** | **int** | The timestamp of the price update. | 

## Example

```python
from openapi_client.models.market_price_update import MarketPriceUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of MarketPriceUpdate from a JSON string
market_price_update_instance = MarketPriceUpdate.from_json(json)
# print the JSON string representation of the object
print(MarketPriceUpdate.to_json())

# convert the object into a dict
market_price_update_dict = market_price_update_instance.to_dict()
# create an instance of MarketPriceUpdate from a dict
market_price_update_from_dict = MarketPriceUpdate.from_dict(market_price_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


