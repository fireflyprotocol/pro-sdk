# AccountFundingRateHistoryData


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payment_amount_e9** | **str** | Payment amount in e9 format. | 
**position_side** | [**PositionSide**](PositionSide.md) |  | 
**rate_e9** | **str** | Funding rate value (e9 format). | 
**symbol** | **str** | Market address. | 
**executed_at_millis** | **int** | Execution timestamp in milliseconds since Unix epoch. | 
**computed_at_millis** | **int** | Computed timestamp in milliseconds since Unix epoch. | 

## Example

```python
from openapi_client.models.account_funding_rate_history_data import AccountFundingRateHistoryData

# TODO update the JSON string below
json = "{}"
# create an instance of AccountFundingRateHistoryData from a JSON string
account_funding_rate_history_data_instance = AccountFundingRateHistoryData.from_json(json)
# print the JSON string representation of the object
print(AccountFundingRateHistoryData.to_json())

# convert the object into a dict
account_funding_rate_history_data_dict = account_funding_rate_history_data_instance.to_dict()
# create an instance of AccountFundingRateHistoryData from a dict
account_funding_rate_history_data_from_dict = AccountFundingRateHistoryData.from_dict(account_funding_rate_history_data_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


