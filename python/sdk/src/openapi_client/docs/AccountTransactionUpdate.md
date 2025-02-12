# AccountTransactionUpdate

Details about a transaction in the account.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | The symbol of the market. | [optional] 
**transaction_type** | [**TransactionType**](TransactionType.md) |  | 
**amount_e9** | **str** | The amount of the transaction in scientific notation with 9 decimal places. | 
**asset_symbol** | **str** | The symbol of the asset. | [optional] 
**trade_id** | **str** | The trade ID associated with the transaction. | [optional] 

## Example

```python
from openapi_client.models.account_transaction_update import AccountTransactionUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of AccountTransactionUpdate from a JSON string
account_transaction_update_instance = AccountTransactionUpdate.from_json(json)
# print the JSON string representation of the object
print(AccountTransactionUpdate.to_json())

# convert the object into a dict
account_transaction_update_dict = account_transaction_update_instance.to_dict()
# create an instance of AccountTransactionUpdate from a dict
account_transaction_update_from_dict = AccountTransactionUpdate.from_dict(account_transaction_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


