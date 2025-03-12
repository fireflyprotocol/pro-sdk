# Account


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trading_fees** | [**TradingFees**](TradingFees.md) |  | 
**can_trade** | **bool** | If the user can trade. | 
**can_deposit** | **bool** | If the current user can deposit to the account. | 
**can_withdraw** | **bool** | If the current user can withdraw from the account. | 
**total_effective_balance_e9** | **str** | Total effective balance in USD (e9 format). | 
**total_initial_margin_required_e9** | **str** | The sum of initial margin required across all cross positions (e9 format). | 
**total_open_order_initial_margin_required_e9** | **str** | The sum of initial margin required across all open orders (e9 format). | 
**initial_margin_available_e9** | **str** | The amount of margin available to open new positions and orders (e9 format). | 
**total_maintenance_margin_required_e9** | **str** | The sum of maintenance margin required across all cross positions (e9 format). | 
**maintenance_margin_available_e9** | **str** | The amount of margin available before liquidation (e9 format). | 
**account_maintenance_margin_ratio_e9** | **str** | The ratio of the maintenance margin required to the account value (e9 format). | 
**account_leverage_e9** | **str** | The leverage of the account (e9 format). | 
**total_unrealized_pnl_e9** | **str** | Total unrealized profit (e9 format). | 
**total_cross_unrealized_pnl_e9** | **str** | Unrealized profit of crossed positions (e9 format). | 
**last_updated_at_millis** | **int** | Last update time in milliseconds since Unix epoch. | 
**assets** | [**List[Asset]**](Asset.md) |  | 
**positions** | [**List[Position]**](Position.md) |  | 
**authorized_accounts** | **List[str]** | The accounts that are authorized to trade on behalf of the current account. | 

## Example

```python
from openapi_client.models.account import Account

# TODO update the JSON string below
json = "{}"
# create an instance of Account from a JSON string
account_instance = Account.from_json(json)
# print the JSON string representation of the object
print(Account.to_json())

# convert the object into a dict
account_dict = account_instance.to_dict()
# create an instance of Account from a dict
account_from_dict = Account.from_dict(account_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


