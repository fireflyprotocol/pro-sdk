# Transaction


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** | Transaction ID. | 
**symbol** | **str** | Market address. | [optional] 
**type** | [**TransactionType**](TransactionType.md) |  | 
**amount_e9** | **str** | Amount in e9 format (positive or negative). | 
**status** | **str** | Transaction status (SUCCESS, REJECTED). | [optional] 
**asset_symbol** | **str** | Asset bank address. | 
**trade_id** | **str** | Trade ID | [optional] 
**executed_at_millis** | **int** | Transaction timestamp in milliseconds since Unix epoch. | 

## Example

```python
from openapi_client.models.transaction import Transaction

# TODO update the JSON string below
json = "{}"
# create an instance of Transaction from a JSON string
transaction_instance = Transaction.from_json(json)
# print the JSON string representation of the object
print(Transaction.to_json())

# convert the object into a dict
transaction_dict = transaction_instance.to_dict()
# create an instance of Transaction from a dict
transaction_from_dict = Transaction.from_dict(transaction_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


