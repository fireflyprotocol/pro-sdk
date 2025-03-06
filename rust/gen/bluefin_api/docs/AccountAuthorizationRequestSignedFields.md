# AccountAuthorizationRequestSignedFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_address** | **String** | The account address of the parent account that is authorizing/deauthorizing this account | 
**authorized_account_address** | **String** | The address of the account that should be authorized/deauthorized | 
**salt** | **String** | The random generated salt. Should always be a number | 
**ids_id** | **String** | the ID of the internal datastore for the target network | 
**signed_at_utc_millis** | **i64** | The timestamp when the request was signed | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


