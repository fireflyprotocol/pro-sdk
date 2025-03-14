# CreateOrderRequestSignedFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | The symbol of the perpetual for which to create the order | 
**account_address** | **String** | The account address of the order. May be an account user is authorized for. | 
**price_e9** | **String** | The price in base e9 of the asset to be traded. Should always be a number | 
**quantity_e9** | **String** | The quantity in base e9 of the asset to be traded. Should always be a number | 
**side** | [**models::OrderSide**](OrderSide.md) |  | 
**leverage_e9** | **String** | The leverage in base e9 of the order to be traded. Should always be a number | 
**is_isolated** | **bool** | Is this order isolated or cross margin. Note market must be set to the same mode. | [default to false]
**salt** | **String** | The random generated SALT. Should always be a number | 
**ids_id** | **String** | the ID of the internal datastore for the target network | 
**expires_at_millis** | **i64** | timestamp in millis at which order will expire. Defaults to 1 month for LIMIT orders if not provided | 
**signed_at_millis** | **i64** | The timestamp in millis at which the request was signed | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


