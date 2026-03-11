# ClientCredentialsRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | **str** |  | 
**client_secret** | **str** |  | 
**grant_type** | **str** |  | 

## Example

```python
from openapi_client.models.client_credentials_request import ClientCredentialsRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ClientCredentialsRequest from a JSON string
client_credentials_request_instance = ClientCredentialsRequest.from_json(json)
# print the JSON string representation of the object
print(ClientCredentialsRequest.to_json())

# convert the object into a dict
client_credentials_request_dict = client_credentials_request_instance.to_dict()
# create an instance of ClientCredentialsRequest from a dict
client_credentials_request_from_dict = ClientCredentialsRequest.from_dict(client_credentials_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


