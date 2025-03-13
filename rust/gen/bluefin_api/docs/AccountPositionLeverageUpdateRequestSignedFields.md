# AccountPositionLeverageUpdateRequestSignedFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_address** | **String** | The Account Address from which to update leverage | 
**symbol** | **String** | Symbol of the perpetual of the positions for which to update the leverage | 
**leverage_e9** | **String** | The leverage to set for the account positions (Must be a number in base e9) | 
**salt** | **String** | The random generated SALT. Should always be a number | 
**ids_id** | **String** | the ID of the internal datastore for the target network | 
**signed_at_millis** | **i64** | The timestamp in millis at which the request was signed | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


