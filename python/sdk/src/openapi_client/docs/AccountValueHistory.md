# AccountValueHistory


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**latest_value_e9** | **str** | Latest account value (e9 format). | 
**value_change_e9** | **str** | Change in value from the first to the last value in the interval (e9 format). | 
**value_change_percentage_e9** | **str** | Percentage change in value from the first to the last value in the interval (e9 format). | 
**latest_unrealized_pnl_e9** | **str** | Latest unrealized PnL value (e9 format). | 
**unrealized_pnl_change_e9** | **str** | Change in unrealized PnL from the first to the last value in the interval (e9 format). | 
**unrealized_pnl_change_percentage_e9** | **str** | Percentage change in unrealized PnL from the first to the last value in the interval (e9 format). | 
**values** | [**List[AccountValueHistoryData]**](AccountValueHistoryData.md) |  | 

## Example

```python
from openapi_client.models.account_value_history import AccountValueHistory

# TODO update the JSON string below
json = "{}"
# create an instance of AccountValueHistory from a JSON string
account_value_history_instance = AccountValueHistory.from_json(json)
# print the JSON string representation of the object
print(AccountValueHistory.to_json())

# convert the object into a dict
account_value_history_dict = account_value_history_instance.to_dict()
# create an instance of AccountValueHistory from a dict
account_value_history_from_dict = AccountValueHistory.from_dict(account_value_history_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


