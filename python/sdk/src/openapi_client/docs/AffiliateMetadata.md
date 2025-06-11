# AffiliateMetadata


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | **str** | The user&#39;s wallet address | 
**parent_referral_code** | **str** | The referral code of the parent affiliate | [optional] 
**referral_code** | **str** | The user&#39;s referral code if approved as an affiliate | [optional] 
**name** | **str** | The name of the affiliate | [optional] 
**parent_name** | **str** | The name of the parent affiliate | [optional] 
**fees** | [**FeeConfigs**](.md) |  | [optional] 
**status** | **str** | Status of the affiliate application | [optional] 
**has_traded** | **bool** | Indicates whether the user has traded or not | 
**tier** | **str** | Tier of the affiliate | [optional] 

## Example

```python
from openapi_client.models.affiliate_metadata import AffiliateMetadata

# TODO update the JSON string below
json = "{}"
# create an instance of AffiliateMetadata from a JSON string
affiliate_metadata_instance = AffiliateMetadata.from_json(json)
# print the JSON string representation of the object
print(AffiliateMetadata.to_json())

# convert the object into a dict
affiliate_metadata_dict = affiliate_metadata_instance.to_dict()
# create an instance of AffiliateMetadata from a dict
affiliate_metadata_from_dict = AffiliateMetadata.from_dict(affiliate_metadata_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


