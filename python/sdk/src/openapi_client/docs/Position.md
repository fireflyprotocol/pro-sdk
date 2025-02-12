# Position


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | Market address. | 
**avg_entry_price_e9** | **str** | Average entry price determined by a simple average of all entry prices resulting in this position size (e9 format). | 
**leverage_e9** | **str** | Isolated position leverage (e9 format). | 
**liquidation_price_e9** | **str** | Liquidation price (e9 format). | 
**mark_price_e9** | **str** | Mark price (e9 format). | 
**notional_value_e9** | **str** | Notional value (e9 format). | 
**max_notional_value_e9** | **str** | Max notional value at current leverage (e9 format). | 
**position_size_e9** | **str** | Position size (e9 format). | 
**unrealized_pnl_e9** | **str** | Unrealized profit (e9 format). | 
**position_side** | [**PositionSideEnum**](PositionSideEnum.md) |  | 
**initial_margin_e9** | **str** | Initial margin required with current mark price (e9 format). | 
**maint_margin_e9** | **str** | Maintenance margin required with current mark price (e9 format). | 
**is_isolated** | **bool** | If the position is isolated. | 
**isolated_margin_e9** | **str** | Margin value present if margin type is isolated (e9 format). | 
**last_updated_at_utc_millis** | **int** | Last update time. | 

## Example

```python
from openapi_client.models.position import Position

# TODO update the JSON string below
json = "{}"
# create an instance of Position from a JSON string
position_instance = Position.from_json(json)
# print the JSON string representation of the object
print(Position.to_json())

# convert the object into a dict
position_dict = position_instance.to_dict()
# create an instance of Position from a dict
position_from_dict = Position.from_dict(position_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


