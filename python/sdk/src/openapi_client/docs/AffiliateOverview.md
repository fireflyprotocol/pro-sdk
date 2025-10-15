# AffiliateOverview


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | **str** | The user&#39;s wallet address | 
**name** | **str** | Name of the affiliate | [optional] 
**perps_referee_earnings_e9** | **str** | Referee earnings from perps trading (e9 format) | 
**spot_lp_referee_earnings_e9** | **str** | Referee earnings from spot LP (e9 format) | 
**lending_referee_earnings_e9** | **str** | Referee earnings from lending (e9 format) | 
**ember_referee_earnings_e9** | **str** | Referee earnings from ember (e9 format) | 
**perps_referral_earnings_e9** | **str** | Referral earnings from perps trading (e9 format) | 
**spot_lp_referral_earnings_e9** | **str** | Referral earnings from spot LP (e9 format) | 
**lending_referral_earnings_e9** | **str** | Referral earnings from lending (e9 format) | 
**perps_total_earnings_e9** | **str** | Total earnings from perps trading (e9 format) | 
**spot_lp_total_earnings_e9** | **str** | Total earnings from spot LP (e9 format) | 
**lending_total_earnings_e9** | **str** | Total earnings from lending (e9 format) | 
**ember_total_earnings_e9** | **str** | Total earnings from ember (e9 format) | 
**total_referral_earnings_e9** | **str** | Total earnings from referrals (e9 format) | 
**total_referee_earnings_e9** | **str** | Total earnings from referee activities (e9 format) | 
**total_earnings_e9** | **str** | Total earnings combining referrals and referee activities (e9 format) | 

## Example

```python
from openapi_client.models.affiliate_overview import AffiliateOverview

# TODO update the JSON string below
json = "{}"
# create an instance of AffiliateOverview from a JSON string
affiliate_overview_instance = AffiliateOverview.from_json(json)
# print the JSON string representation of the object
print(AffiliateOverview.to_json())

# convert the object into a dict
affiliate_overview_dict = affiliate_overview_instance.to_dict()
# create an instance of AffiliateOverview from a dict
affiliate_overview_from_dict = AffiliateOverview.from_dict(affiliate_overview_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


