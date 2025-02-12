# CreateOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signed_fields** | [**models::CreateOrderRequestSignedFields**](CreateOrderRequest_signedFields.md) |  | 
**signature** | **String** | The signature of the request, encoded from the signedFields | 
**order_hash** | **String** | The identifier of this order used for lookup. This should always be unique. Created by hex encoding the bcs encoded signedFields. | 
**client_order_id** | Option<**String**> | The client-defined unique identifier of this order used for lookup. This should always be unique; however, the server will not gurantee this or impose any checks. | [optional]
**r#type** | [**models::OrderType**](OrderType.md) |  | 
**reduce_only** | **bool** | Is this order to only reduce a position? Default false | 
**post_only** | **bool** | If set to TRUE, the order can only be a maker order | 
**time_in_force** | [**models::OrderTimeInForce**](OrderTimeInForce.md) |  | 
**trigger_price_e9** | Option<**String**> | Trigger price in base e9 for stop orders. This should always be a number | [optional]
**self_trade_prevention_type** | Option<[**models::SelfTradePreventionType**](SelfTradePreventionType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


