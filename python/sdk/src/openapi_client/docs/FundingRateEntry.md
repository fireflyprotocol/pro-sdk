# FundingRateEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | The market symbol. | 
**funding_time_at_millis** | **int** | Timestamp of the funding time in milliseconds. | 
**funding_rate_e9** | **str** | Funding rate for the market address. | 

## Example

```python
from openapi_client.models.funding_rate_entry import FundingRateEntry

# TODO update the JSON string below
json = "{}"
# create an instance of FundingRateEntry from a JSON string
funding_rate_entry_instance = FundingRateEntry.from_json(json)
# print the JSON string representation of the object
print(FundingRateEntry.to_json())

# convert the object into a dict
funding_rate_entry_dict = funding_rate_entry_instance.to_dict()
# create an instance of FundingRateEntry from a dict
funding_rate_entry_from_dict = FundingRateEntry.from_dict(funding_rate_entry_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


