# WithdrawRequestSignedFields


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_symbol** | **str** | Asset symbol of the withdrawn asset | 
**account_address** | **str** | The Account Address from which to withdraw assets | 
**amount_e9** | **str** | The amount in e9 of the asset that the User will withdraw from their account | 
**salt** | **str** | A uniqueness modifier for the request. This is added to guarantee uniqueness of the request. Usually a mix of timestamp and a random number | 
**eds_id** | **str** | the ID of the external datastore for the target network | 
**signed_at_millis** | **int** | The timestamp in milliseconds when the HTTP Request payload has been signed | 

## Example

```python
from openapi_client.models.withdraw_request_signed_fields import WithdrawRequestSignedFields

# TODO update the JSON string below
json = "{}"
# create an instance of WithdrawRequestSignedFields from a JSON string
withdraw_request_signed_fields_instance = WithdrawRequestSignedFields.from_json(json)
# print the JSON string representation of the object
print(WithdrawRequestSignedFields.to_json())

# convert the object into a dict
withdraw_request_signed_fields_dict = withdraw_request_signed_fields_instance.to_dict()
# create an instance of WithdrawRequestSignedFields from a dict
withdraw_request_signed_fields_from_dict = WithdrawRequestSignedFields.from_dict(withdraw_request_signed_fields_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


