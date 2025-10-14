# StatsEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_time_at_millis** | **int** | Timestamp in milliseconds when the statistics period starts. | 
**legacy_tvl_e9** | **str** | Total value locked in the exchange in e9 format. | 
**tvl_e9** | **str** | Total value locked in the exchange in e9 format. | 
**total_legacy_quote_asset_volume_e9** | **str** | Total quote asset volume in the legacy exchange in e9 format. | 
**total_quote_asset_volume_e9** | **str** | Total quote asset volume in the exchange in e9 format. | 
**end_time_at_millis** | **int** | Timestamp in milliseconds when the statistics period ends. | 

## Example

```python
from openapi_client.models.stats_entry import StatsEntry

# TODO update the JSON string below
json = "{}"
# create an instance of StatsEntry from a JSON string
stats_entry_instance = StatsEntry.from_json(json)
# print the JSON string representation of the object
print(StatsEntry.to_json())

# convert the object into a dict
stats_entry_dict = stats_entry_instance.to_dict()
# create an instance of StatsEntry from a dict
stats_entry_from_dict = StatsEntry.from_dict(stats_entry_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


