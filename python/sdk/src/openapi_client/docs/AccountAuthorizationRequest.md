# AccountAuthorizationRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signed_fields** | [**AccountAuthorizationRequestSignedFields**](AccountAuthorizationRequestSignedFields.md) |  | 
**signature** | **str** | The signature of the request, encoded from the signedFields | 
**request_hash** | **str** | Used to uniquely identify the request. Created by hex encoding the bcs encoded signedFields. | 

## Example

```python
from openapi_client.models.account_authorization_request import AccountAuthorizationRequest

# TODO update the JSON string below
json = "{}"
# create an instance of AccountAuthorizationRequest from a JSON string
account_authorization_request_instance = AccountAuthorizationRequest.from_json(json)
# print the JSON string representation of the object
print(AccountAuthorizationRequest.to_json())

# convert the object into a dict
account_authorization_request_dict = account_authorization_request_instance.to_dict()
# create an instance of AccountAuthorizationRequest from a dict
account_authorization_request_from_dict = AccountAuthorizationRequest.from_dict(account_authorization_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


