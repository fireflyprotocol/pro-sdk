# AccountUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_id** | Option<**String**> | The optional group ID of the account. | [optional]
**trading_fees** | Option<[**models::TradingFees**](TradingFees.md)> |  | [optional]
**can_trade** | **bool** | If the user can trade. | 
**can_deposit** | **bool** | If the current user can deposit to the account. | 
**can_withdraw** | **bool** | If the current user can withdraw from the account. | 
**cross_effective_balance_e9** | **String** | Total effective balance in USD (e9 format). | 
**cross_margin_required_e9** | **String** | The sum of initial margin required across all cross positions (e9 format). | 
**total_order_margin_required_e9** | **String** | The sum of initial margin required across all open orders (e9 format). | 
**margin_available_e9** | **String** | The amount of margin available to open new positions and orders (e9 format). | 
**cross_maintenance_margin_required_e9** | **String** | The sum of maintenance margin required across all cross positions (e9 format). | 
**cross_maintenance_margin_available_e9** | **String** | The amount of margin available before liquidation (e9 format). | 
**cross_maintenance_margin_ratio_e9** | **String** | The ratio of the maintenance margin required to the account value (e9 format). | 
**cross_leverage_e9** | **String** | The leverage of the account (e9 format). | 
**total_unrealized_pnl_e9** | **String** | Total unrealized profit (e9 format). | 
**cross_unrealized_pnl_e9** | **String** | Unrealized profit of cross positions (e9 format). | 
**cross_unrealized_loss_e9** | **String** | An implicitly negative number that sums only the losses of all cross positions. | 
**cross_account_value_e9** | **String** | The total value of the cross account, combining the cross effective balance and unrealized PnL across all cross positions, and subtracting any pending funding payments on any cross position.  | 
**total_account_value_e9** | **String** | The total value of the account, combining the total effective balance and unrealized PnL across all positions, and subtracting any pending funding payments on any position.  | 
**updated_at_millis** | **i64** | Last update time in milliseconds since Unix epoch. | 
**assets** | [**Vec<models::Asset>**](Asset.md) |  | 
**authorized_accounts** | **Vec<String>** | Deprecated: Replaced with authorizedWallets. | 
**authorized_wallets** | [**Vec<models::AuthorizedWallet>**](AuthorizedWallet.md) | The wallets that are authorized to trade on behalf of the current account. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


