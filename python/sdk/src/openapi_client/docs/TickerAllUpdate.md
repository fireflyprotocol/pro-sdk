# TickerAllUpdate


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker_all** | [**List[TickerUpdate]**](TickerUpdate.md) | Array of detailed market ticker information for all markets. | 

## Example

```python
from openapi_client.models.ticker_all_update import TickerAllUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of TickerAllUpdate from a JSON string
ticker_all_update_instance = TickerAllUpdate.from_json(json)
# print the JSON string representation of the object
print(TickerAllUpdate.to_json())

# convert the object into a dict
ticker_all_update_dict = ticker_all_update_instance.to_dict()
# create an instance of TickerAllUpdate from a dict
ticker_all_update_from_dict = TickerAllUpdate.from_dict(ticker_all_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


