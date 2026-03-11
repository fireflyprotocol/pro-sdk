# VaultClaimResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tx_hash** | **str** | The claimed transaction hash. | 
**status** | **str** | Claim status. | 

## Example

```python
from openapi_client.models.vault_claim_response import VaultClaimResponse

# TODO update the JSON string below
json = "{}"
# create an instance of VaultClaimResponse from a JSON string
vault_claim_response_instance = VaultClaimResponse.from_json(json)
# print the JSON string representation of the object
print(VaultClaimResponse.to_json())

# convert the object into a dict
vault_claim_response_dict = vault_claim_response_instance.to_dict()
# create an instance of VaultClaimResponse from a dict
vault_claim_response_from_dict = VaultClaimResponse.from_dict(vault_claim_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


