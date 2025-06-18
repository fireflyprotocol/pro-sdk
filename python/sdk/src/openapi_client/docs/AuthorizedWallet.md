# AuthorizedWallet


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **str** | The address of the authorized wallet. | 
**alias** | **str** | The alias of the authorized wallet. | [optional] 
**authorized_at_millis** | **int** | The timestamp in milliseconds when the wallet was authorized. | 

## Example

```python
from openapi_client.models.authorized_wallet import AuthorizedWallet

# TODO update the JSON string below
json = "{}"
# create an instance of AuthorizedWallet from a JSON string
authorized_wallet_instance = AuthorizedWallet.from_json(json)
# print the JSON string representation of the object
print(AuthorizedWallet.to_json())

# convert the object into a dict
authorized_wallet_dict = authorized_wallet_instance.to_dict()
# create an instance of AuthorizedWallet from a dict
authorized_wallet_from_dict = AuthorizedWallet.from_dict(authorized_wallet_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


