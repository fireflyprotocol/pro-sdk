# TradingFees


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**maker_fee_e9** | **str** | The Account Maker Fee (e9 format). | 
**taker_fee_e9** | **str** | The Account Taker Fee (e9 format). | 
**is_applied** | **bool** | Are the fees applied on the account? | 

## Example

```python
from openapi_client.models.trading_fees import TradingFees

# TODO update the JSON string below
json = "{}"
# create an instance of TradingFees from a JSON string
trading_fees_instance = TradingFees.from_json(json)
# print the JSON string representation of the object
print(TradingFees.to_json())

# convert the object into a dict
trading_fees_dict = trading_fees_instance.to_dict()
# create an instance of TradingFees from a dict
trading_fees_from_dict = TradingFees.from_dict(trading_fees_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


