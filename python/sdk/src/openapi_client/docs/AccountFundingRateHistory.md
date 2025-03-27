# AccountFundingRateHistory


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | [**List[AccountFundingRateHistoryData]**](AccountFundingRateHistoryData.md) |  | 

## Example

```python
from openapi_client.models.account_funding_rate_history import AccountFundingRateHistory

# TODO update the JSON string below
json = "{}"
# create an instance of AccountFundingRateHistory from a JSON string
account_funding_rate_history_instance = AccountFundingRateHistory.from_json(json)
# print the JSON string representation of the object
print(AccountFundingRateHistory.to_json())

# convert the object into a dict
account_funding_rate_history_dict = account_funding_rate_history_instance.to_dict()
# create an instance of AccountFundingRateHistory from a dict
account_funding_rate_history_from_dict = AccountFundingRateHistory.from_dict(account_funding_rate_history_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


