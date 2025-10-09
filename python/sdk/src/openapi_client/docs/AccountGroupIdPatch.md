# AccountGroupIdPatch


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_address** | **str** | The address of the account to update. | 
**group_id** | **str** | The new group to assign the account to. If not present, the account will be removed from any group.  | [optional] 

## Example

```python
from openapi_client.models.account_group_id_patch import AccountGroupIdPatch

# TODO update the JSON string below
json = "{}"
# create an instance of AccountGroupIdPatch from a JSON string
account_group_id_patch_instance = AccountGroupIdPatch.from_json(json)
# print the JSON string representation of the object
print(AccountGroupIdPatch.to_json())

# convert the object into a dict
account_group_id_patch_dict = account_group_id_patch_instance.to_dict()
# create an instance of AccountGroupIdPatch from a dict
account_group_id_patch_from_dict = AccountGroupIdPatch.from_dict(account_group_id_patch_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


