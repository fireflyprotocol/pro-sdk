# Position

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | Market address. | 
**avg_entry_price_e9** | **String** | Average entry price determined by a simple average of all entry prices resulting in this position size (e9 format). | 
**leverage_e9** | **String** | Isolated position leverage (e9 format). | 
**liquidation_price_e9** | **String** | Liquidation price (e9 format). | 
**mark_price_e9** | **String** | Mark price (e9 format). | 
**notional_value_e9** | **String** | Notional value (e9 format). | 
**size_e9** | **String** | Position size (e9 format). | 
**unrealized_pnl_e9** | **String** | Unrealized profit (e9 format). | 
**side** | [**models::PositionSide**](PositionSide.md) |  | 
**initial_margin_e9** | **String** | Initial margin required with current mark price (e9 format). | 
**maintenance_margin_e9** | **String** | Maintenance margin required with current mark price (e9 format). | 
**is_isolated** | **bool** | If the position is isolated. | 
**isolated_margin_e9** | **String** | Margin value present if margin type is isolated (e9 format). | 
**updated_at_millis** | **i64** | Last update time. | 
**funding_rate_payment_all_time_e9** | **String** | Total funding rate payment (e9 format). | 
**funding_rate_payment_since_change_e9** | **String** | Funding rate payment since last position change (e9 format). | 
**funding_rate_payment_since_opened_e9** | **String** | Funding rate payment since position opened (e9 format). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


