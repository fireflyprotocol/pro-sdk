# ZKLoginUserDetailsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**salt** | **str** | The zkLogin user salt. | 
**address** | **str** | The zkLogin user&#39;s address. | 
**public_key** | **str** | The zkLogin user&#39;s public key. | 

## Example

```python
from openapi_client.models.zk_login_user_details_response import ZKLoginUserDetailsResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ZKLoginUserDetailsResponse from a JSON string
zk_login_user_details_response_instance = ZKLoginUserDetailsResponse.from_json(json)
# print the JSON string representation of the object
print(ZKLoginUserDetailsResponse.to_json())

# convert the object into a dict
zk_login_user_details_response_dict = zk_login_user_details_response_instance.to_dict()
# create an instance of ZKLoginUserDetailsResponse from a dict
zk_login_user_details_response_from_dict = ZKLoginUserDetailsResponse.from_dict(zk_login_user_details_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


