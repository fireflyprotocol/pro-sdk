# AccountTradeUpdate

Details about a trade in the account.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trade_id** | **str** | The unique identifier for the trade. | 
**client_order_id** | **str** | The client order ID associated with the trade. | [optional] 
**symbol** | **str** | The symbol of the market. | 
**order_hash** | **str** | The hash of the order. | 
**type** | [**TradeType**](TradeType.md) |  | 
**order_side** | [**Side**](Side.md) |  | 
**is_maker** | **bool** | Indicates if the trade was a maker order. | 
**price_e9** | **str** | The price of the trade. | 
**quantity_e9** | **str** | The quantity of the trade. | 
**quote_quantity_e9** | **str** | The quote quantity of the trade. | 
**realized_pnl_e9** | **str** | The realized profit and loss. | 
**position_side** | [**Side**](Side.md) |  | 
**trading_fee_e9** | **str** | The trading fee for the trade. | 
**trading_fee_asset_symbol** | **str** | The market symbol of the asset used for the trading fee. | 
**executed_at_millis** | **int** | The timestamp when the trade was executed in milliseconds. | 

## Example

```python
from openapi_client.models.account_trade_update import AccountTradeUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of AccountTradeUpdate from a JSON string
account_trade_update_instance = AccountTradeUpdate.from_json(json)
# print the JSON string representation of the object
print(AccountTradeUpdate.to_json())

# convert the object into a dict
account_trade_update_dict = account_trade_update_instance.to_dict()
# create an instance of AccountTradeUpdate from a dict
account_trade_update_from_dict = AccountTradeUpdate.from_dict(account_trade_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


