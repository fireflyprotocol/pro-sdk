# AccountAuthorizationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signed_fields** | [**models::AccountAuthorizationRequestSignedFields**](AccountAuthorizationRequest_signedFields.md) |  | 
**signature** | **String** | The signature of the request, encoded from the signedFields | 
**request_hash** | **String** | Used to uniquely identify the request. Created by hex encoding the bcs encoded signedFields. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


