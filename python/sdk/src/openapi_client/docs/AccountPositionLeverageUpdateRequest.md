# AccountPositionLeverageUpdateRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signed_fields** | [**AccountPositionLeverageUpdateRequestSignedFields**](AccountPositionLeverageUpdateRequestSignedFields.md) |  | 
**signature** | **str** | The signature of the request, encoded from the signedFields | 

## Example

```python
from openapi_client.models.account_position_leverage_update_request import AccountPositionLeverageUpdateRequest

# TODO update the JSON string below
json = "{}"
# create an instance of AccountPositionLeverageUpdateRequest from a JSON string
account_position_leverage_update_request_instance = AccountPositionLeverageUpdateRequest.from_json(json)
# print the JSON string representation of the object
print(AccountPositionLeverageUpdateRequest.to_json())

# convert the object into a dict
account_position_leverage_update_request_dict = account_position_leverage_update_request_instance.to_dict()
# create an instance of AccountPositionLeverageUpdateRequest from a dict
account_position_leverage_update_request_from_dict = AccountPositionLeverageUpdateRequest.from_dict(account_position_leverage_update_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


