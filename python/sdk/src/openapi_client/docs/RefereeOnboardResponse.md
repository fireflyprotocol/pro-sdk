# RefereeOnboardResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **str** | Response message indicating if the referral code exists | 

## Example

```python
from openapi_client.models.referee_onboard_response import RefereeOnboardResponse

# TODO update the JSON string below
json = "{}"
# create an instance of RefereeOnboardResponse from a JSON string
referee_onboard_response_instance = RefereeOnboardResponse.from_json(json)
# print the JSON string representation of the object
print(RefereeOnboardResponse.to_json())

# convert the object into a dict
referee_onboard_response_dict = referee_onboard_response_instance.to_dict()
# create an instance of RefereeOnboardResponse from a dict
referee_onboard_response_from_dict = RefereeOnboardResponse.from_dict(referee_onboard_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


