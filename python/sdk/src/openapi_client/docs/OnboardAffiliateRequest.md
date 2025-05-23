# OnboardAffiliateRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parent_referral_code** | **str** | Referral code of the parent affiliate | [optional] 
**name** | **str** | Name of the applicant | 
**email** | **str** | Email address of the applicant | 
**social_user_names** | [**OnboardAffiliateRequestSocialUserNames**](OnboardAffiliateRequestSocialUserNames.md) |  | [optional] 

## Example

```python
from openapi_client.models.onboard_affiliate_request import OnboardAffiliateRequest

# TODO update the JSON string below
json = "{}"
# create an instance of OnboardAffiliateRequest from a JSON string
onboard_affiliate_request_instance = OnboardAffiliateRequest.from_json(json)
# print the JSON string representation of the object
print(OnboardAffiliateRequest.to_json())

# convert the object into a dict
onboard_affiliate_request_dict = onboard_affiliate_request_instance.to_dict()
# create an instance of OnboardAffiliateRequest from a dict
onboard_affiliate_request_from_dict = OnboardAffiliateRequest.from_dict(onboard_affiliate_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


