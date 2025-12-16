# ContractConfig


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin_cap** | **str** |  | [optional] 
**package** | **str** |  | [optional] 
**upgrade_cap** | **str** |  | [optional] 
**rewards_pool** | [**Dict[str, RewardsPoolEntry]**](RewardsPoolEntry.md) |  | [optional] 

## Example

```python
from openapi_client.models.contract_config import ContractConfig

# TODO update the JSON string below
json = "{}"
# create an instance of ContractConfig from a JSON string
contract_config_instance = ContractConfig.from_json(json)
# print the JSON string representation of the object
print(ContractConfig.to_json())

# convert the object into a dict
contract_config_dict = contract_config_instance.to_dict()
# create an instance of ContractConfig from a dict
contract_config_from_dict = ContractConfig.from_dict(contract_config_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


