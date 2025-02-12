# FeeTier

Details about the fee tier.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**maker_fee_e9** | **str** | The maker fee. | 
**taker_fee_e9** | **str** | The taker fee. | 
**is_applied** | **bool** | Indicates if the fee tier is applied. | 

## Example

```python
from openapi_client.models.fee_tier import FeeTier

# TODO update the JSON string below
json = "{}"
# create an instance of FeeTier from a JSON string
fee_tier_instance = FeeTier.from_json(json)
# print the JSON string representation of the object
print(FeeTier.to_json())

# convert the object into a dict
fee_tier_dict = fee_tier_instance.to_dict()
# create an instance of FeeTier from a dict
fee_tier_from_dict = FeeTier.from_dict(fee_tier_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


