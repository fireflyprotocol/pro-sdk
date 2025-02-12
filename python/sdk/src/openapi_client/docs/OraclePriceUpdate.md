# OraclePriceUpdate


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | The symbol of the market. | 
**price_e9** | **str** | The price in scientific notation with 9 decimal places of precision. | 
**source** | **str** |  | 
**updated_at_utc_millis** | **int** | The timestamp of the price update. | 

## Example

```python
from openapi_client.models.oracle_price_update import OraclePriceUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of OraclePriceUpdate from a JSON string
oracle_price_update_instance = OraclePriceUpdate.from_json(json)
# print the JSON string representation of the object
print(OraclePriceUpdate.to_json())

# convert the object into a dict
oracle_price_update_dict = oracle_price_update_instance.to_dict()
# create an instance of OraclePriceUpdate from a dict
oracle_price_update_from_dict = OraclePriceUpdate.from_dict(oracle_price_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


