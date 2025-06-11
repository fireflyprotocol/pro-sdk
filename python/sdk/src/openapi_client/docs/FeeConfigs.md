# FeeConfigs

Map of various fee-related configurations

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**referral_perps_fee_e9** | **str** | Earnings from referral perps fees (e9 format) | [optional] 
**subaffiliate_perps_earnings_e9** | **str** | Earnings from subaffiliate perps (e9 format) | [optional] 
**spot_lp_fee_e9** | **str** | Earnings from spot LP fees (e9 format) | [optional] 
**referral_spot_lp_fee_e9** | **str** | Earnings from referral spot LP fees (e9 format) | [optional] 
**referral_lending_rewards_e9** | **str** | Earnings from referral lending rewards (e9 format) | [optional] 
**perps_fee_cashback_e9** | **str** | Cashback from perps fees (e9 format) | [optional] 

## Example

```python
from openapi_client.models.fee_configs import FeeConfigs

# TODO update the JSON string below
json = "{}"
# create an instance of FeeConfigs from a JSON string
fee_configs_instance = FeeConfigs.from_json(json)
# print the JSON string representation of the object
print(FeeConfigs.to_json())

# convert the object into a dict
fee_configs_dict = fee_configs_instance.to_dict()
# create an instance of FeeConfigs from a dict
fee_configs_from_dict = FeeConfigs.from_dict(fee_configs_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


