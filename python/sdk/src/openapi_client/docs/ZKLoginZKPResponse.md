# ZKLoginZKPResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**proof_points** | [**ProofPoints**](ProofPoints.md) |  | [optional] 
**iss_base64_details** | [**IssBase64Details**](IssBase64Details.md) |  | [optional] 
**header_base64** | **str** | Base64 encoded header information. | [optional] 
**address_seed** | **str** | The address seed used in the proof. | 

## Example

```python
from openapi_client.models.zk_login_zkp_response import ZKLoginZKPResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ZKLoginZKPResponse from a JSON string
zk_login_zkp_response_instance = ZKLoginZKPResponse.from_json(json)
# print the JSON string representation of the object
print(ZKLoginZKPResponse.to_json())

# convert the object into a dict
zk_login_zkp_response_dict = zk_login_zkp_response_instance.to_dict()
# create an instance of ZKLoginZKPResponse from a dict
zk_login_zkp_response_from_dict = ZKLoginZKPResponse.from_dict(zk_login_zkp_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


