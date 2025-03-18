# AccountAuthorizationRequestSignedFields


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_address** | **str** | The account address of the parent account that is authorizing/deauthorizing this account | 
**authorized_account_address** | **str** | The address of the account that should be authorized/deauthorized | 
**salt** | **str** | The random generated salt. Should always be a number | 
**ids_id** | **str** | the ID of the internal datastore for the target network | 
**signed_at_millis** | **int** | The timestamp when the request was signed | 

## Example

```python
from openapi_client.models.account_authorization_request_signed_fields import AccountAuthorizationRequestSignedFields

# TODO update the JSON string below
json = "{}"
# create an instance of AccountAuthorizationRequestSignedFields from a JSON string
account_authorization_request_signed_fields_instance = AccountAuthorizationRequestSignedFields.from_json(json)
# print the JSON string representation of the object
print(AccountAuthorizationRequestSignedFields.to_json())

# convert the object into a dict
account_authorization_request_signed_fields_dict = account_authorization_request_signed_fields_instance.to_dict()
# create an instance of AccountAuthorizationRequestSignedFields from a dict
account_authorization_request_signed_fields_from_dict = AccountAuthorizationRequestSignedFields.from_dict(account_authorization_request_signed_fields_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


