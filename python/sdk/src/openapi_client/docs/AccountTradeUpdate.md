# AccountTradeUpdate

Details about a trade in the account.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trade** | [**Trade**](Trade.md) |  | 

## Example

```python
from openapi_client.models.account_trade_update import AccountTradeUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of AccountTradeUpdate from a JSON string
account_trade_update_instance = AccountTradeUpdate.from_json(json)
# print the JSON string representation of the object
print(AccountTradeUpdate.to_json())

# convert the object into a dict
account_trade_update_dict = account_trade_update_instance.to_dict()
# create an instance of AccountTradeUpdate from a dict
account_trade_update_from_dict = AccountTradeUpdate.from_dict(account_trade_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


