# ClaimSignatureItem


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reward_type** | **str** | Type of reward for this claim signature. | 
**sig_payload** | [**SigPayload**](SigPayload.md) |  | 
**signature** | **str** | Signature for the claim. | 

## Example

```python
from openapi_client.models.claim_signature_item import ClaimSignatureItem

# TODO update the JSON string below
json = "{}"
# create an instance of ClaimSignatureItem from a JSON string
claim_signature_item_instance = ClaimSignatureItem.from_json(json)
# print the JSON string representation of the object
print(ClaimSignatureItem.to_json())

# convert the object into a dict
claim_signature_item_dict = claim_signature_item_instance.to_dict()
# create an instance of ClaimSignatureItem from a dict
claim_signature_item_from_dict = ClaimSignatureItem.from_dict(claim_signature_item_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


