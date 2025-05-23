# AffiliateMetadataFees

Map of various fee-related configurations

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**referral_perps_fee** | **str** | Earnings from referral perps fees (e9 format) | [optional] 
**subaffiliate_perps_earnings** | **str** | Earnings from subaffiliate perps (e9 format) | [optional] 
**spot_lp_fee** | **str** | Earnings from spot LP fees (e9 format) | [optional] 
**referral_spot_lp_fee** | **str** | Earnings from referral spot LP fees (e9 format) | [optional] 
**referral_lending_rewards** | **str** | Earnings from referral lending rewards (e9 format) | [optional] 
**perps_fee_cashback** | **str** | Cashback from perps fees (e9 format) | [optional] 

## Example

```python
from openapi_client.models.affiliate_metadata_fees import AffiliateMetadataFees

# TODO update the JSON string below
json = "{}"
# create an instance of AffiliateMetadataFees from a JSON string
affiliate_metadata_fees_instance = AffiliateMetadataFees.from_json(json)
# print the JSON string representation of the object
print(AffiliateMetadataFees.to_json())

# convert the object into a dict
affiliate_metadata_fees_dict = affiliate_metadata_fees_instance.to_dict()
# create an instance of AffiliateMetadataFees from a dict
affiliate_metadata_fees_from_dict = AffiliateMetadataFees.from_dict(affiliate_metadata_fees_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


