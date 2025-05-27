# Trade

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Trade ID | 
**client_order_id** | Option<**String**> | Client order ID. | [optional]
**symbol** | Option<**String**> | Market address. | [optional]
**order_hash** | Option<**String**> | Order hash. | [optional]
**order_type** | Option<[**models::OrderType**](OrderType.md)> |  | [optional]
**trade_type** | Option<[**models::TradeType**](TradeType.md)> |  | [optional]
**side** | [**models::TradeSide**](TradeSide.md) |  | 
**is_maker** | Option<**bool**> | Indicates if the user was a maker to the trade. | [optional]
**price_e9** | **String** | Trade price (e9 format). | 
**quantity_e9** | **String** | Trade quantity (e9 format). | 
**quote_quantity_e9** | **String** | Quote quantity (e9 format). | 
**realized_pnl_e9** | Option<**String**> | Realized profit and loss (e9 format). | [optional]
**position_side** | Option<[**models::PositionSide**](PositionSide.md)> |  | [optional]
**trading_fee_e9** | Option<**String**> | Trading fee (e9 format). | [optional]
**trading_fee_asset** | Option<**String**> | Asset used for trading fee. | [optional]
**gas_fee_e9** | Option<**String**> | Gas fee. | [optional]
**gas_fee_asset** | Option<**String**> | Asset used for gas fee. | [optional]
**executed_at_millis** | **i64** | Trade timestamp in milliseconds since Unix epoch. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


