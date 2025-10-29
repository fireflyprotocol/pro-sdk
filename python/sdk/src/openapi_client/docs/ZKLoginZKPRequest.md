# ZKLoginZKPRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**network** | **str** | The network to use (e.g., \&quot;mainnet\&quot;, \&quot;testnet\&quot;). | [optional] 
**ephemeral_public_key** | **str** | The ephemeral public key for the ZK proof. | 
**max_epoch** | **int** | The maximum epoch for the ZK proof. | 
**randomness** | **str** | Randomness value for the ZK proof. | 

## Example

```python
from openapi_client.models.zk_login_zkp_request import ZKLoginZKPRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ZKLoginZKPRequest from a JSON string
zk_login_zkp_request_instance = ZKLoginZKPRequest.from_json(json)
# print the JSON string representation of the object
print(ZKLoginZKPRequest.to_json())

# convert the object into a dict
zk_login_zkp_request_dict = zk_login_zkp_request_instance.to_dict()
# create an instance of ZKLoginZKPRequest from a dict
zk_login_zkp_request_from_dict = ZKLoginZKPRequest.from_dict(zk_login_zkp_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


