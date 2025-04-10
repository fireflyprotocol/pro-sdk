# AdjustIsolatedMarginRequestSignedFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ids_id** | **String** | the ID of the internal datastore for the target network | 
**account_address** | **String** | The account address of the account for which to adjust margin | 
**symbol** | **String** | The symbol of the isolated position for which to adjust margin | 
**operation** | [**models::AdjustMarginOperation**](AdjustMarginOperation.md) |  | 
**quantity_e9** | **String** | The quantity of margin to adjust for the isolated position | 
**salt** | **String** | The random generated SALT. Should always be a number | 
**signed_at_millis** | **i64** | The timestamp in millis at which the request was signed | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


