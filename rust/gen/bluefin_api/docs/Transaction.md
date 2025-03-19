# Transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Transaction ID. | 
**symbol** | Option<**String**> | Market address. | [optional]
**r#type** | [**models::TransactionType**](TransactionType.md) |  | 
**amount_e9** | **String** | Amount in e9 format (positive or negative). | 
**asset_symbol** | **String** | Asset bank address. | 
**trade_id** | Option<**String**> | Trade ID | [optional]
**executed_at_millis** | **i64** | Transaction timestamp in milliseconds since Unix epoch. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


