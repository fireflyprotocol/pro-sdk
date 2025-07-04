# AccountPositionUpdate

Details about an account position update.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | The symbol of the market. | 
**avg_entry_price_e9** | **str** | The average entry price for the position. | 
**client_set_leverage_e9** | **str** | The leverage applied to the position. | 
**liquidation_price_e9** | **str** | The liquidation price of the position. | 
**mark_price_e9** | **str** | The current mark price of the position. | 
**notional_value_e9** | **str** | The notional value of the position. | 
**size_e9** | **str** | The size of the position. | 
**unrealized_pnl_e9** | **str** | The unrealized profit and loss for the position. | 
**side** | [**PositionSide**](PositionSide.md) |  | 
**margin_required_e9** | **str** | The margin required for the position. | 
**maintenance_margin_e9** | **str** | The maintenance margin required for the position. | 
**is_isolated** | **bool** | Indicates if the position is isolated. | 
**isolated_margin_e9** | **str** | The isolated margin applied to the position. | 
**updated_at_millis** | **int** | The last update time for the position in milliseconds. | 

## Example

```python
from openapi_client.models.account_position_update import AccountPositionUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of AccountPositionUpdate from a JSON string
account_position_update_instance = AccountPositionUpdate.from_json(json)
# print the JSON string representation of the object
print(AccountPositionUpdate.to_json())

# convert the object into a dict
account_position_update_dict = account_position_update_instance.to_dict()
# create an instance of AccountPositionUpdate from a dict
account_position_update_from_dict = AccountPositionUpdate.from_dict(account_position_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


