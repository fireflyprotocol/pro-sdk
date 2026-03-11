# SwapClaimRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tx_hash** | **str** | On-chain transaction hash of the swap. | 

## Example

```python
from openapi_client.models.swap_claim_request import SwapClaimRequest

# TODO update the JSON string below
json = "{}"
# create an instance of SwapClaimRequest from a JSON string
swap_claim_request_instance = SwapClaimRequest.from_json(json)
# print the JSON string representation of the object
print(SwapClaimRequest.to_json())

# convert the object into a dict
swap_claim_request_dict = swap_claim_request_instance.to_dict()
# create an instance of SwapClaimRequest from a dict
swap_claim_request_from_dict = SwapClaimRequest.from_dict(swap_claim_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


