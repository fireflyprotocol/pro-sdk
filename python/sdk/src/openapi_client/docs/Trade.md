# Trade


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** | Trade ID | 
**client_order_id** | **str** | Client order ID. | [optional] 
**symbol** | **str** | Market address. | [optional] 
**order_hash** | **str** | Order hash. | [optional] 
**trade_type** | [**TradeType**](TradeType.md) |  | [optional] 
**side** | [**TradeSide**](TradeSide.md) |  | 
**is_maker** | **bool** | Indicates if the user was a maker to the trade. | [optional] 
**price_e9** | **str** | Trade price (e9 format). | 
**quantity_e9** | **str** | Trade quantity (e9 format). | 
**quote_quantity_e9** | **str** | Quote quantity (e9 format). | 
**realized_pnl_e9** | **str** | Realized profit and loss (e9 format). | [optional] 
**position_side** | [**PositionSide**](PositionSide.md) |  | [optional] 
**trading_fee_e9** | **str** | Trading fee (e9 format). | [optional] 
**trading_fee_asset** | **str** | Asset used for trading fee. | [optional] 
**gas_fee_e9** | **float** | Gas fee. | [optional] 
**gas_fee_asset** | **str** | Asset used for gas fee. | [optional] 
**executed_at_millis** | **int** | Trade timestamp in milliseconds since Unix epoch. | 

## Example

```python
from openapi_client.models.trade import Trade

# TODO update the JSON string below
json = "{}"
# create an instance of Trade from a JSON string
trade_instance = Trade.from_json(json)
# print the JSON string representation of the object
print(Trade.to_json())

# convert the object into a dict
trade_dict = trade_instance.to_dict()
# create an instance of Trade from a dict
trade_from_dict = Trade.from_dict(trade_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


