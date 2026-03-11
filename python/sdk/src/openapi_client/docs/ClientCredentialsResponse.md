# ClientCredentialsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token** | **str** |  | 
**token_type** | **str** |  | 
**expires_in** | **int** | Token validity in seconds | 

## Example

```python
from openapi_client.models.client_credentials_response import ClientCredentialsResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ClientCredentialsResponse from a JSON string
client_credentials_response_instance = ClientCredentialsResponse.from_json(json)
# print the JSON string representation of the object
print(ClientCredentialsResponse.to_json())

# convert the object into a dict
client_credentials_response_dict = client_credentials_response_instance.to_dict()
# create an instance of ClientCredentialsResponse from a dict
client_credentials_response_from_dict = ClientCredentialsResponse.from_dict(client_credentials_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


