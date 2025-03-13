# LoginRequest

User is expected to sign this payload and sends is signature in login api as header and payload itself in request body 

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_address** | **str** | The address of the account. | 
**signed_at_millis** | **int** | The timestamp in millis when the login was signed. | 
**audience** | **str** | The intended audience of the login request. | 

## Example

```python
from openapi_client.models.login_request import LoginRequest

# TODO update the JSON string below
json = "{}"
# create an instance of LoginRequest from a JSON string
login_request_instance = LoginRequest.from_json(json)
# print the JSON string representation of the object
print(LoginRequest.to_json())

# convert the object into a dict
login_request_dict = login_request_instance.to_dict()
# create an instance of LoginRequest from a dict
login_request_from_dict = LoginRequest.from_dict(login_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


