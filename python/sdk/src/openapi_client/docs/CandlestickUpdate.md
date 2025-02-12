# CandlestickUpdate

Represents a candlestick for a specific market and interval.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | The symbol of the market for this candlestick. | 
**start_time** | **int** | The start time of the candlestick in milliseconds since epoch. | 
**end_time** | **int** | The end time of the candlestick in milliseconds since epoch. | 
**interval** | **str** | The interval of the candlestick (e.g., 1m, 5m, 1h). | 
**open_price_e9** | **str** | The opening price of the candlestick. | 
**close_price_e9** | **str** | The closing price of the candlestick. | 
**high_price_e9** | **str** | The highest price during the candlestick interval. | 
**low_price_e9** | **str** | The lowest price during the candlestick interval. | 
**volume_e9** | **str** | The total trading volume during the candlestick interval. | 
**quote_volume_e9** | **str** | The total quote volume traded during the candlestick interval. | 
**num_trades** | **int** | The number of trades that occurred during the candlestick interval. | 

## Example

```python
from openapi_client.models.candlestick_update import CandlestickUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of CandlestickUpdate from a JSON string
candlestick_update_instance = CandlestickUpdate.from_json(json)
# print the JSON string representation of the object
print(CandlestickUpdate.to_json())

# convert the object into a dict
candlestick_update_dict = candlestick_update_instance.to_dict()
# create an instance of CandlestickUpdate from a dict
candlestick_update_from_dict = CandlestickUpdate.from_dict(candlestick_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


