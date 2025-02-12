# Account

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trading_fees** | [**models::TradingFees**](TradingFees.md) |  | 
**can_trade** | **bool** | If the user can trade. | 
**can_deposit** | **bool** | If the current user can deposit to the account. | 
**can_withdraw** | **bool** | If the current user can withdraw from the account. | 
**total_effective_balance_e9** | **String** | Total effective balance in USD (e9 format). | 
**total_initial_margin_required_e9** | **String** | The sum of initial margin required across all cross positions (e9 format). | 
**total_open_order_initial_margin_required_e9** | **String** | The sum of initial margin required across all open orders (e9 format). | 
**initial_margin_available_e9** | **String** | The amount of margin available to open new positions and orders (e9 format). | 
**total_maint_margin_required_e9** | **String** | The sum of maintenance margin required across all cross positions (e9 format). | 
**maint_margin_available_e9** | **String** | The amount of margin available before liquidation (e9 format). | 
**account_maint_margin_ratio_e9** | **String** | The ratio of the maintenance margin required to the account value (e9 format). | 
**account_leverage_e9** | **String** | The leverage of the account (e9 format). | 
**total_unrealized_pnl_e9** | **String** | Total unrealized profit (e9 format). | 
**total_cross_unrealized_pnl_e9** | **String** | Unrealized profit of crossed positions (e9 format). | 
**last_updated_at_utc_millis** | **i64** | Last update time in milliseconds since Unix epoch. | 
**assets** | [**Vec<models::Asset>**](Asset.md) |  | 
**positions** | [**Vec<models::Position>**](Position.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


