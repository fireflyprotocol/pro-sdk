# ExchangeInfoResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assets** | [**Vec<models::Asset1>**](Asset_1.md) | List of assets available on the exchange. | 
**contracts_config** | Option<[**models::ContractsConfig**](ContractsConfig.md)> |  | [optional]
**markets** | [**Vec<models::Market>**](Market.md) | List of markets available on the exchange. | 
**trading_gas_fee_e9** | **String** | Current gas fee set for subsidized trades (e9 format) | 
**server_time_at_utc_millis** | **i64** | Server time in milliseconds. | 
**timezone** | **String** | Timezone of the exchange. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


