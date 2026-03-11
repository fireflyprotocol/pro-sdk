# VaultClaimRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tx_hash** | **str** | On-chain transaction hash of the vault deposit. | 

## Example

```python
from openapi_client.models.vault_claim_request import VaultClaimRequest

# TODO update the JSON string below
json = "{}"
# create an instance of VaultClaimRequest from a JSON string
vault_claim_request_instance = VaultClaimRequest.from_json(json)
# print the JSON string representation of the object
print(VaultClaimRequest.to_json())

# convert the object into a dict
vault_claim_request_dict = vault_claim_request_instance.to_dict()
# create an instance of VaultClaimRequest from a dict
vault_claim_request_from_dict = VaultClaimRequest.from_dict(vault_claim_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


