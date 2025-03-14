# ActiveOrderUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_hash** | **String** | The unique hash of the order. | 
**client_order_id** | Option<**String**> | The client-provided order ID. | [optional]
**symbol** | **String** | The symbol of the market. | 
**account_address** | **String** | The address of the account. | 
**price_e9** | **String** | The price of the order in scientific notation with 9 decimal places. | 
**quantity_e9** | **String** | The quantity of the order in scientific notation with 9 decimal places. | 
**filled_quantity_e9** | **String** | The filled quantity of the order in scientific notation with 9 decimal places. | 
**side** | [**models::Side**](Side.md) |  | 
**leverage_e9** | **String** | The leverage of the order in scientific notation with 9 decimal places. | 
**is_isolated** | **bool** | Indicates if the order is isolated. | 
**salt** | **String** | A unique salt for the order. | 
**expires_at_millis** | **i64** | The expiration timestamp of the order in milliseconds. | 
**signed_at_millis** | **i64** | The signing timestamp of the order in milliseconds. | 
**r#type** | [**models::OrderType1**](OrderType_1.md) |  | 
**reduce_only** | **bool** | Indicates if the order is reduce-only. | 
**post_only** | **bool** | Indicates if the order is post-only. | 
**time_in_force** | [**models::OrderTimeInForce1**](OrderTimeInForce_1.md) |  | 
**trigger_price_e9** | Option<**String**> | The trigger price for stop-limit or stop-market orders. | [optional]
**status** | [**models::OrderStatus1**](OrderStatus_1.md) |  | 
**self_trade_prevention_type** | [**models::SelfTradePreventionType1**](SelfTradePreventionType_1.md) |  | 
**created_at_millis** | **i64** | The timestamp when the order was placed, in milliseconds. | 
**updated_at_millis** | **i64** | The timestamp of the last update of the order in milliseconds. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


