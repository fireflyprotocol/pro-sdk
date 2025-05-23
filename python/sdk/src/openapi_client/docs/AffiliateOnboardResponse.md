# AffiliateOnboardResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **str** | Status of the application | 
**message** | **str** | Response message including rejection reason if application was rejected | 

## Example

```python
from openapi_client.models.affiliate_onboard_response import AffiliateOnboardResponse

# TODO update the JSON string below
json = "{}"
# create an instance of AffiliateOnboardResponse from a JSON string
affiliate_onboard_response_instance = AffiliateOnboardResponse.from_json(json)
# print the JSON string representation of the object
print(AffiliateOnboardResponse.to_json())

# convert the object into a dict
affiliate_onboard_response_dict = affiliate_onboard_response_instance.to_dict()
# create an instance of AffiliateOnboardResponse from a dict
affiliate_onboard_response_from_dict = AffiliateOnboardResponse.from_dict(affiliate_onboard_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


