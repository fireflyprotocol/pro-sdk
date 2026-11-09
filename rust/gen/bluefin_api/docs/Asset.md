# Asset

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | The symbol of the asset. | 
**quantity_e9** | **String** | The quantity of the asset. | 
**effective_balance_e9** | **String** | The effective balance of the asset. | 
**max_withdraw_quantity_e9** | **String** | The maximum quantity that can be withdrawn, in e9. It is not rounded to the asset's on-chain decimals; round it down to a multiple of 10^(9 - decimals) (1000 for USDC) before using it as a withdrawal amount. | 
**updated_at_millis** | **i64** | The timestamp of the last update in milliseconds. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


