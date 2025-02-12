# ContractsConfig

Contract configuration for the exchange.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**eds_id** | **str** | External Data Store Address | 
**ids_id** | **str** | External Data Store Address | 
**network** | **str** | Network environment | 
**base_contract_address** | **str** | Base contract address | 
**current_contract_address** | **str** | Current contract address | 
**operators** | [**Operators**](Operators.md) |  | 

## Example

```python
from openapi_client.models.contracts_config import ContractsConfig

# TODO update the JSON string below
json = "{}"
# create an instance of ContractsConfig from a JSON string
contracts_config_instance = ContractsConfig.from_json(json)
# print the JSON string representation of the object
print(ContractsConfig.to_json())

# convert the object into a dict
contracts_config_dict = contracts_config_instance.to_dict()
# create an instance of ContractsConfig from a dict
contracts_config_from_dict = ContractsConfig.from_dict(contracts_config_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


