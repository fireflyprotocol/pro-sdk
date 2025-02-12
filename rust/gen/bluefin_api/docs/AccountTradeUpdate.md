# AccountTradeUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trade_id** | **String** | The unique identifier for the trade. | 
**client_order_id** | Option<**String**> | The client order ID associated with the trade. | [optional]
**symbol** | **String** | The symbol of the market. | 
**order_hash** | **String** | The hash of the order. | 
**r#type** | [**models::TradeType**](TradeType.md) |  | 
**order_side** | [**models::Side**](Side.md) |  | 
**is_maker** | **bool** | Indicates if the trade was a maker order. | 
**price_e9** | **String** | The price of the trade. | 
**quantity_e9** | **String** | The quantity of the trade. | 
**quote_quantity_e9** | **String** | The quote quantity of the trade. | 
**realized_pnl_e9** | **String** | The realized profit and loss. | 
**position_side** | [**models::Side**](Side.md) |  | 
**trading_fee_e9** | **String** | The trading fee for the trade. | 
**trading_fee_asset_symbol** | **String** | The market symbol of the asset used for the trading fee. | 
**executed_at_utc_millis** | **i64** | The timestamp when the trade was executed in milliseconds. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


