# OrderCancellationUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_hash** | **String** | The unique hash of the order. | 
**client_order_id** | Option<**String**> | The client-provided order ID. | [optional]
**symbol** | **String** | The symbol of the market. | 
**account_address** | **String** | The address of the account. | 
**created_at_millis** | **i64** | The timestamp of the order creation in milliseconds. | 
**cancellation_reason** | [**models::OrderCancelReason**](OrderCancelReason.md) |  | 
**failure_to_cancel_reason** | Option<[**models::OrderCancellationFailureReason**](OrderCancellationFailureReason.md)> |  | [optional]
**remaining_quantity_e9** | **String** | The remaining quantity of the order. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


