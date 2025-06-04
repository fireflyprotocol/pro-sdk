# UpdateAccountPreferenceRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**language** | **str** | User preferred language. | [optional] 
**theme** | **str** | User preferred theme. | [optional] 
**market** | [**List[AccountMarketPreference]**](AccountMarketPreference.md) |  | [optional] 

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


