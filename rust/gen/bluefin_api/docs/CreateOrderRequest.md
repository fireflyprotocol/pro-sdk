# CreateOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signed_fields** | [**models::CreateOrderRequestSignedFields**](CreateOrderRequest_signedFields.md) |  | 
**signature** | **String** | The signature of the request, encoded from the signedFields | 
**client_order_id** | Option<**String**> | The client-defined unique identifier of this order used for lookup. This should always be unique; however, the server will not gurantee this or impose any checks. | [optional]
**r#type** | [**models::OrderType**](OrderType.md) |  | 
**reduce_only** | **bool** | Is this order to only reduce a position? Default false | 
**post_only** | Option<**bool**> | If set to TRUE, the order can only be a maker order | [optional][default to false]
**time_in_force** | Option<[**models::OrderTimeInForce**](OrderTimeInForce.md)> | Omit or set to null for market orders; otherwise, choose a valid time-in-force value. GTT: Good Til Time  IOC: Immediate Or Cancel  FOK: Fill Or Kill  | [optional]
**trigger_price_e9** | Option<**String**> | Trigger price in base e9 for stop orders. This should always be a number | [optional]
**self_trade_prevention_type** | Option<[**models::SelfTradePreventionType**](SelfTradePreventionType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


