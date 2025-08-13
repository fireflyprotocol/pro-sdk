# SponsorTxResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tx_bytes** | **str** | Base64 encoded sponsored transaction bytes | 
**tx_digest** | **str** | Transaction digest | 
**signature** | **str** | Transaction signature | 
**expire_at_time** | **int** | Transaction expiration time in milliseconds since Unix epoch | 

## Example

```python
from openapi_client.models.sponsor_tx_response import SponsorTxResponse

# TODO update the JSON string below
json = "{}"
# create an instance of SponsorTxResponse from a JSON string
sponsor_tx_response_instance = SponsorTxResponse.from_json(json)
# print the JSON string representation of the object
print(SponsorTxResponse.to_json())

# convert the object into a dict
sponsor_tx_response_dict = sponsor_tx_response_instance.to_dict()
# create an instance of SponsorTxResponse from a dict
sponsor_tx_response_from_dict = SponsorTxResponse.from_dict(sponsor_tx_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


