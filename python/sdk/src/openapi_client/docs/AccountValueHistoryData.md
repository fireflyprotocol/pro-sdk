# AccountValueHistoryData


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timestamp_millis** | **int** | Timestamp in milliseconds since Unix epoch. | 
**unrealized_pnl_e9** | **str** | Unrealized PnL at this timestamp (e9 format). | 
**value_e9** | **str** | Account value at this timestamp (e9 format). | 

## Example

```python
from openapi_client.models.account_value_history_data import AccountValueHistoryData

# TODO update the JSON string below
json = "{}"
# create an instance of AccountValueHistoryData from a JSON string
account_value_history_data_instance = AccountValueHistoryData.from_json(json)
# print the JSON string representation of the object
print(AccountValueHistoryData.to_json())

# convert the object into a dict
account_value_history_data_dict = account_value_history_data_instance.to_dict()
# create an instance of AccountValueHistoryData from a dict
account_value_history_data_from_dict = AccountValueHistoryData.from_dict(account_value_history_data_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


