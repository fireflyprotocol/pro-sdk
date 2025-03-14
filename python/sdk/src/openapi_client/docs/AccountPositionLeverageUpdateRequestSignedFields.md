# AccountPositionLeverageUpdateRequestSignedFields


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_address** | **str** | The Account Address from which to update leverage | 
**symbol** | **str** | Symbol of the perpetual of the positions for which to update the leverage | 
**leverage_e9** | **str** | The leverage to set for the account positions (Must be a number in base e9) | 
**salt** | **str** | The random generated SALT. Should always be a number | 
**ids_id** | **str** | the ID of the internal datastore for the target network | 
**signed_at_millis** | **int** | The timestamp in millis at which the request was signed | 

## Example

```python
from openapi_client.models.account_position_leverage_update_request_signed_fields import AccountPositionLeverageUpdateRequestSignedFields

# TODO update the JSON string below
json = "{}"
# create an instance of AccountPositionLeverageUpdateRequestSignedFields from a JSON string
account_position_leverage_update_request_signed_fields_instance = AccountPositionLeverageUpdateRequestSignedFields.from_json(json)
# print the JSON string representation of the object
print(AccountPositionLeverageUpdateRequestSignedFields.to_json())

# convert the object into a dict
account_position_leverage_update_request_signed_fields_dict = account_position_leverage_update_request_signed_fields_instance.to_dict()
# create an instance of AccountPositionLeverageUpdateRequestSignedFields from a dict
account_position_leverage_update_request_signed_fields_from_dict = AccountPositionLeverageUpdateRequestSignedFields.from_dict(account_position_leverage_update_request_signed_fields_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


