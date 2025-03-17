# WithdrawRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signed_fields** | [**WithdrawRequestSignedFields**](WithdrawRequestSignedFields.md) |  | 
**signature** | **str** | The signature of the request, encoded from the signedFields | 

## Example

```python
from openapi_client.models.withdraw_request import WithdrawRequest

# TODO update the JSON string below
json = "{}"
# create an instance of WithdrawRequest from a JSON string
withdraw_request_instance = WithdrawRequest.from_json(json)
# print the JSON string representation of the object
print(WithdrawRequest.to_json())

# convert the object into a dict
withdraw_request_dict = withdraw_request_instance.to_dict()
# create an instance of WithdrawRequest from a dict
withdraw_request_from_dict = WithdrawRequest.from_dict(withdraw_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


