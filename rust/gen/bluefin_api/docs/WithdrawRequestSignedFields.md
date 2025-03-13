# WithdrawRequestSignedFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_symbol** | **String** | Asset symbol of the withdrawn asset | 
**account_address** | **String** | The Account Address from which to withdraw assets | 
**amount_e9** | **String** | The amount in e9 of the asset that the User will withdraw from their account | 
**salt** | **String** | A uniqueness modifier for the request. This is added to guarantee uniqueness of the request. Usually a mix of timestamp and a random number | 
**eds_id** | **String** | the ID of the external datastore for the target network | 
**signed_at_millis** | **i64** | The timestamp in milliseconds when the HTTP Request payload has been signed | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


