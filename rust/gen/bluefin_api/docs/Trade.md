# Trade

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trade_id** | **String** | Trade ID | 
**client_order_id** | Option<**String**> | Client order ID. | [optional]
**symbol** | Option<**String**> | Market address. | [optional]
**order_hash** | Option<**String**> | Order hash. | [optional]
**trade_type** | [**models::TradeTypeEnum**](TradeTypeEnum.md) |  | 
**side** | [**models::TradeSideEnum**](TradeSideEnum.md) |  | 
**is_maker** | **bool** | Indicates if the user was a maker to the trade. | 
**price_e9** | **String** | Trade price (e9 format). | 
**quantity_e9** | **String** | Trade quantity (e9 format). | 
**quote_quantity_e9** | **String** | Quote quantity (e9 format). | 
**realized_pnl_e9** | Option<**String**> | Realized profit and loss (e9 format). | [optional]
**position_side** | [**models::PositionSideEnum**](PositionSideEnum.md) |  | 
**trading_fee_e9** | **String** | Trading fee (e9 format). | 
**trading_fee_asset** | **String** | Asset used for trading fee. | 
**gas_fee_e9** | Option<**f32**> | Gas fee. | [optional]
**gas_fee_asset** | Option<**String**> | Asset used for gas fee. | [optional]
**executed_at_millis** | **i64** | Trade timestamp in milliseconds since Unix epoch. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


