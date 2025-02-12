# OpenOrderResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_hash** | **String** | The Order Hash, which is the default way to uniquely identify an order in the system | 
**client_order_id** | Option<**String**> | The Client Order ID, which is used a unique identifier for an order, provided by the client, in case of proprietary order management systems | [optional]
**symbol** | **String** | The market symbol | 
**account_address** | **String** | The account address of the order. May be an account user is authorized for. | 
**price_e9** | **String** | The price in base e9 of the asset to be traded. Should always be a number | 
**quantity_e9** | **String** | The quantity in base e9 of the asset to be traded. Should always be a number | 
**side** | [**models::OrderSide**](OrderSide.md) |  | 
**leverage_e9** | **String** | The leverage in base e9  of the order to be traded. Should always be a number | 
**is_isolated** | **bool** | Is this order isolated or cross margin. Note market must be set to the same mode. | [default to false]
**salt** | **String** | The random generated SALT. Should always be a number | 
**expires_at_utc_millis** | **i64** | Unix timestamp in millis at which order will expire. Defaults to 1 month for LIMIT orders if not provided | 
**signed_at_utc_millis** | **i64** | The timestamp in millis at which the request was signed | 
**r#type** | [**models::OrderType**](OrderType.md) |  | 
**reduce_only** | **bool** | Is this order to only reduce a position? Default false | [default to false]
**post_only** | **bool** | If set to TRUE, the order can only be a maker order | [default to false]
**time_in_force** | [**models::OrderTimeInForce**](OrderTimeInForce.md) |  | 
**trigger_price_e9** | Option<**String**> | Trigger price in base e9 for stop orders. This should always be a number | [optional]
**filled_quantity_e9** | **String** | The quantity in base e9 of the asset currently filled. This should always be a number | 
**status** | [**models::OrderStatus**](OrderStatus.md) |  | 
**self_trade_prevention_type** | [**models::SelfTradePreventionType**](SelfTradePreventionType.md) |  | 
**order_time_at_utc_millis** | **i64** | The timestamp in millis when the order was opened | 
**last_updated_at_utc_millis** | **i64** | The timestamp in millis that this order was last updated (including status updates) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


