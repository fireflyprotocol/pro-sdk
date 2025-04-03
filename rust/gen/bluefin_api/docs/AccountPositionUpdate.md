# AccountPositionUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | The symbol of the market. | 
**avg_entry_price_e9** | **String** | The average entry price for the position. | 
**leverage_e9** | **String** | The leverage applied to the position. | 
**liquidation_price_e9** | **String** | The liquidation price of the position. | 
**mark_price_e9** | **String** | The current mark price of the position. | 
**notional_value_e9** | **String** | The notional value of the position. | 
**size_e9** | **String** | The size of the position. | 
**unrealized_pnl_e9** | **String** | The unrealized profit and loss for the position. | 
**side** | [**models::PositionSide**](PositionSide.md) |  | 
**initial_margin_e9** | **String** | The initial margin required for the position. | 
**maintenance_margin_e9** | **String** | The maintenance margin required for the position. | 
**is_isolated** | **bool** | Indicates if the position is isolated. | 
**isolated_margin_e9** | **String** | The isolated margin applied to the position. | 
**updated_at_millis** | **i64** | The last update time for the position in milliseconds. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


