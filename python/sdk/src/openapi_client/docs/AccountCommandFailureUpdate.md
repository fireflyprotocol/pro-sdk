# AccountCommandFailureUpdate

Details about a failure during an account command execution.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reason** | **str** | The reason for the failure. | 
**failed_command_type** | **str** | The type of command that failed. | 
**failed_at_millis** | **int** | The timestamp when the command failed in milliseconds. | 

## Example

```python
from openapi_client.models.account_command_failure_update import AccountCommandFailureUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of AccountCommandFailureUpdate from a JSON string
account_command_failure_update_instance = AccountCommandFailureUpdate.from_json(json)
# print the JSON string representation of the object
print(AccountCommandFailureUpdate.to_json())

# convert the object into a dict
account_command_failure_update_dict = account_command_failure_update_instance.to_dict()
# create an instance of AccountCommandFailureUpdate from a dict
account_command_failure_update_from_dict = AccountCommandFailureUpdate.from_dict(account_command_failure_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


