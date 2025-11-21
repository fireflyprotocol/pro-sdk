# SigPayload


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target** | **str** | Target address for the claim. | 
**receiver** | **str** | Receiver address for the claim. | 
**amount** | **str** | Amount to be claimed. | 
**expiry** | **str** | Expiry timestamp for the claim. | 
**nonce** | **str** | Nonce for the claim. | 
**type** | **int** | Type identifier for the claim. | 

## Example

```python
from openapi_client.models.sig_payload import SigPayload

# TODO update the JSON string below
json = "{}"
# create an instance of SigPayload from a JSON string
sig_payload_instance = SigPayload.from_json(json)
# print the JSON string representation of the object
print(SigPayload.to_json())

# convert the object into a dict
sig_payload_dict = sig_payload_instance.to_dict()
# create an instance of SigPayload from a dict
sig_payload_from_dict = SigPayload.from_dict(sig_payload_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


