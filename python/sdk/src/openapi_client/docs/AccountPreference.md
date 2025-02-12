# AccountPreference


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**language** | **str** | User preferred language. | [optional] 
**theme** | **str** | User preferred theme. | [optional] 
**market** | [**List[AccountMarketPreference]**](AccountMarketPreference.md) |  | [optional] 

## Example

```python
from openapi_client.models.account_preference import AccountPreference

# TODO update the JSON string below
json = "{}"
# create an instance of AccountPreference from a JSON string
account_preference_instance = AccountPreference.from_json(json)
# print the JSON string representation of the object
print(AccountPreference.to_json())

# convert the object into a dict
account_preference_dict = account_preference_instance.to_dict()
# create an instance of AccountPreference from a dict
account_preference_from_dict = AccountPreference.from_dict(account_preference_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


