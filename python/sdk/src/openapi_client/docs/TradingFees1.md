# TradingFees1

Details about the fee tier.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**maker_fee_e9** | **str** | The maker fee. | 
**taker_fee_e9** | **str** | The taker fee. | 
**is_applied** | **bool** | Indicates if the fee tier is applied. | 

## Example

```python
from openapi_client.models.trading_fees1 import TradingFees1

# TODO update the JSON string below
json = "{}"
# create an instance of TradingFees1 from a JSON string
trading_fees1_instance = TradingFees1.from_json(json)
# print the JSON string representation of the object
print(TradingFees1.to_json())

# convert the object into a dict
trading_fees1_dict = trading_fees1_instance.to_dict()
# create an instance of TradingFees1 from a dict
trading_fees1_from_dict = TradingFees1.from_dict(trading_fees1_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


