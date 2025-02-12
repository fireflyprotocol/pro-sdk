# AccountUpdate

Account information for the data stream.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fee_tier** | [**FeeTier**](FeeTier.md) |  | [optional] 
**can_trade** | **bool** | Indicates if trading is enabled. | 
**can_deposit** | **bool** | Indicates if deposits are enabled. | 
**can_withdraw** | **bool** | Indicates if withdrawals are enabled. | 
**total_effective_balance_e9** | **str** | The total effective balance. | 
**total_initial_margin_required_e9** | **str** | The total initial margin required. | 
**total_open_order_initial_margin_required_e9** | **str** | The initial margin required for open orders. | 
**initial_margin_available_e9** | **str** | The available initial margin. | 
**total_maintenance_margin_required_e9** | **str** | The total maintenance margin required. | 
**maintenance_margin_available_e9** | **str** | The available maintenance margin. | 
**account_maintenance_margin_ratio_e9** | **str** | The maintenance margin ratio. | 
**account_leverage_e9** | **str** | The account leverage. | 
**total_unrealized_pnl_e9** | **str** | The total unrealized profit and loss. | 
**total_cross_unrealized_pnl_e9** | **str** | The total cross unrealized profit and loss. | 
**updated_at_utc_millis** | **int** | The timestamp of the last update in milliseconds. | 
**assets** | [**List[Asset2]**](Asset2.md) | The list of assets. | 

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


