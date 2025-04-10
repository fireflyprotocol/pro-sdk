# AdjustIsolatedMarginRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signed_fields** | [**AdjustIsolatedMarginRequestSignedFields**](AdjustIsolatedMarginRequestSignedFields.md) |  | 
**signature** | **str** | The signature of the request, encoded from the signedFields | 

## Example

```python
from openapi_client.models.adjust_isolated_margin_request import AdjustIsolatedMarginRequest

# TODO update the JSON string below
json = "{}"
# create an instance of AdjustIsolatedMarginRequest from a JSON string
adjust_isolated_margin_request_instance = AdjustIsolatedMarginRequest.from_json(json)
# print the JSON string representation of the object
print(AdjustIsolatedMarginRequest.to_json())

# convert the object into a dict
adjust_isolated_margin_request_dict = adjust_isolated_margin_request_instance.to_dict()
# create an instance of AdjustIsolatedMarginRequest from a dict
adjust_isolated_margin_request_from_dict = AdjustIsolatedMarginRequest.from_dict(adjust_isolated_margin_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


