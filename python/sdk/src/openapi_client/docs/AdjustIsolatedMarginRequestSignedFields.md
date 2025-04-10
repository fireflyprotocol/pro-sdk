# AdjustIsolatedMarginRequestSignedFields


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ids_id** | **str** | the ID of the internal datastore for the target network | 
**account_address** | **str** | The account address of the account for which to adjust margin | 
**symbol** | **str** | The symbol of the isolated position for which to adjust margin | 
**operation** | [**AdjustMarginOperation**](AdjustMarginOperation.md) |  | 
**quantity_e9** | **str** | The quantity of margin to adjust for the isolated position | 
**salt** | **str** | The random generated SALT. Should always be a number | 
**signed_at_millis** | **int** | The timestamp in millis at which the request was signed | 

## Example

```python
from openapi_client.models.adjust_isolated_margin_request_signed_fields import AdjustIsolatedMarginRequestSignedFields

# TODO update the JSON string below
json = "{}"
# create an instance of AdjustIsolatedMarginRequestSignedFields from a JSON string
adjust_isolated_margin_request_signed_fields_instance = AdjustIsolatedMarginRequestSignedFields.from_json(json)
# print the JSON string representation of the object
print(AdjustIsolatedMarginRequestSignedFields.to_json())

# convert the object into a dict
adjust_isolated_margin_request_signed_fields_dict = adjust_isolated_margin_request_signed_fields_instance.to_dict()
# create an instance of AdjustIsolatedMarginRequestSignedFields from a dict
adjust_isolated_margin_request_signed_fields_from_dict = AdjustIsolatedMarginRequestSignedFields.from_dict(adjust_isolated_margin_request_signed_fields_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


