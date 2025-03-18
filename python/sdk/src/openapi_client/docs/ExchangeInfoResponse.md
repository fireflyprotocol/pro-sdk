# ExchangeInfoResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assets** | [**List[Asset1]**](Asset1.md) | List of assets available on the exchange. | 
**contracts_config** | [**ContractsConfig**](ContractsConfig.md) |  | [optional] 
**markets** | [**List[Market]**](Market.md) | List of markets available on the exchange. | 
**trading_gas_fee_e9** | **str** | Current gas fee set for subsidized trades (e9 format) | 
**server_time_at_millis** | **int** | Server time in milliseconds. | 
**timezone** | **str** | Timezone of the exchange. | 

## Example

```python
from openapi_client.models.exchange_info_response import ExchangeInfoResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ExchangeInfoResponse from a JSON string
exchange_info_response_instance = ExchangeInfoResponse.from_json(json)
# print the JSON string representation of the object
print(ExchangeInfoResponse.to_json())

# convert the object into a dict
exchange_info_response_dict = exchange_info_response_instance.to_dict()
# create an instance of ExchangeInfoResponse from a dict
exchange_info_response_from_dict = ExchangeInfoResponse.from_dict(exchange_info_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


