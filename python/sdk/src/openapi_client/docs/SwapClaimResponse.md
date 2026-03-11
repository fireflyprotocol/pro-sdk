# SwapClaimResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tx_hash** | **str** | The claimed transaction hash. | 
**status** | **str** | Claim status. | 

## Example

```python
from openapi_client.models.swap_claim_response import SwapClaimResponse

# TODO update the JSON string below
json = "{}"
# create an instance of SwapClaimResponse from a JSON string
swap_claim_response_instance = SwapClaimResponse.from_json(json)
# print the JSON string representation of the object
print(SwapClaimResponse.to_json())

# convert the object into a dict
swap_claim_response_dict = swap_claim_response_instance.to_dict()
# create an instance of SwapClaimResponse from a dict
swap_claim_response_from_dict = SwapClaimResponse.from_dict(swap_claim_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


