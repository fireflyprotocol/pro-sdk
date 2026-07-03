# UpdateAccountPreferenceRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**favorites** | **List[str]** | Favorite market symbols. Send the full array each time; to remove a favorite, send the array without it.  | [optional] 
**function_bar_mode** | **str** | Function bar display mode. Note the mixed casing: &#x60;all&#x60; is lowercase while &#x60;Popular&#x60; and &#x60;Favorites&#x60; are capitalized.  | [optional] 
**onboarding_completed** | **bool** | Whether the user has completed onboarding. | [optional] 
**terms_accepted** | **bool** | Whether the user has accepted the terms of service. | [optional] 

## Example

```python
from openapi_client.models.update_account_preference_request import UpdateAccountPreferenceRequest

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateAccountPreferenceRequest from a JSON string
update_account_preference_request_instance = UpdateAccountPreferenceRequest.from_json(json)
# print the JSON string representation of the object
print(UpdateAccountPreferenceRequest.to_json())

# convert the object into a dict
update_account_preference_request_dict = update_account_preference_request_instance.to_dict()
# create an instance of UpdateAccountPreferenceRequest from a dict
update_account_preference_request_from_dict = UpdateAccountPreferenceRequest.from_dict(update_account_preference_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


