# AccountUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trading_fees** | Option<[**models::TradingFees**](TradingFees.md)> |  | [optional]
**can_trade** | **bool** | Indicates if trading is enabled. | 
**can_deposit** | **bool** | Indicates if deposits are enabled. | 
**can_withdraw** | **bool** | Indicates if withdrawals are enabled. | 
**total_effective_balance_e9** | **String** | The total effective balance. | 
**total_initial_margin_required_e9** | **String** | The total initial margin required. | 
**total_open_order_initial_margin_required_e9** | **String** | The initial margin required for open orders. | 
**initial_margin_available_e9** | **String** | The available initial margin. | 
**total_maintenance_margin_required_e9** | **String** | The total maintenance margin required. | 
**maintenance_margin_available_e9** | **String** | The available maintenance margin. | 
**account_maintenance_margin_ratio_e9** | **String** | The maintenance margin ratio. | 
**account_leverage_e9** | **String** | The account leverage. | 
**total_unrealized_pnl_e9** | **String** | The total unrealized profit and loss. | 
**total_cross_unrealized_pnl_e9** | **String** | The total cross unrealized profit and loss. | 
**updated_at_millis** | **i64** | The timestamp of the last update in milliseconds. | 
**assets** | [**Vec<models::Asset2>**](Asset_2.md) | The list of assets. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


