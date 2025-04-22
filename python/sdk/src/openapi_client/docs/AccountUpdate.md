# AccountUpdate

Account information for the data stream.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trading_fees** | [**TradingFees**](TradingFees.md) |  | [optional] 
**can_trade** | **bool** | If the user can trade. | 
**can_deposit** | **bool** | If the current user can deposit to the account. | 
**can_withdraw** | **bool** | If the current user can withdraw from the account. | 
**cross_effective_balance_e9** | **str** | Total effective balance in USD (e9 format). | 
**cross_margin_required_e9** | **str** | The sum of initial margin required across all cross positions (e9 format). | 
**total_order_margin_required_e9** | **str** | The sum of initial margin required across all open orders (e9 format). | 
**margin_available_e9** | **str** | The amount of margin available to open new positions and orders (e9 format). | 
**cross_maintenance_margin_required_e9** | **str** | The sum of maintenance margin required across all cross positions (e9 format). | 
**cross_maintenance_margin_available_e9** | **str** | The amount of margin available before liquidation (e9 format). | 
**cross_maintenance_margin_ratio_e9** | **str** | The ratio of the maintenance margin required to the account value (e9 format). | 
**cross_leverage_e9** | **str** | The leverage of the account (e9 format). | 
**total_unrealized_pnl_e9** | **str** | Total unrealized profit (e9 format). | 
**cross_unrealized_pnl_e9** | **str** | Unrealized profit of cross positions (e9 format). | 
**cross_unrealized_loss_e9** | **str** | An implicitly negative number that sums only the losses of all cross positions. | 
**cross_account_value_e9** | **str** | The total value of the cross account, combining the cross effective balance and unrealized PnL across all cross positions, and subtracting any pending funding payments on any cross position.  | 
**total_account_value_e9** | **str** | The total value of the account, combining the total effective balance and unrealized PnL across all positions, and subtracting any pending funding payments on any position.  | 
**updated_at_millis** | **int** | Last update time in milliseconds since Unix epoch. | 
**assets** | [**List[Asset]**](Asset.md) |  | 
**authorized_accounts** | **List[str]** | The accounts that are authorized to trade on behalf of the current account. | 

## Example

```python
from openapi_client.models.account_update import AccountUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of AccountUpdate from a JSON string
account_update_instance = AccountUpdate.from_json(json)
# print the JSON string representation of the object
print(AccountUpdate.to_json())

# convert the object into a dict
account_update_dict = account_update_instance.to_dict()
# create an instance of AccountUpdate from a dict
account_update_from_dict = AccountUpdate.from_dict(account_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


