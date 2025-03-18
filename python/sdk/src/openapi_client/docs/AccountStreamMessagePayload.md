# AccountStreamMessagePayload

The payload of the message, which varies based on the event type.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trading_fees** | [**TradingFees**](TradingFees.md) |  | [optional] 
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
**updated_at_millis** | **int** | The last update time for the position in milliseconds. | 
**assets** | [**List[Asset2]**](Asset2.md) | The list of assets. | 
**trade_id** | **str** | The trade ID associated with the transaction. | 
**client_order_id** | **str** | The client-provided order ID. | [optional] 
**symbol** | **str** | The symbol of the market. | 
**order_hash** | **str** | The unique hash of the order. | 
**type** | [**OrderType1**](OrderType1.md) |  | 
**trade_side** | [**Side**](Side.md) |  | 
**is_maker** | **bool** | Indicates if the trade was a maker order. | 
**price_e9** | **str** | The price of the order in scientific notation with 9 decimal places. | 
**quantity_e9** | **str** | The quantity of the order in scientific notation with 9 decimal places. | 
**quote_quantity_e9** | **str** | The quote quantity of the trade. | 
**realized_pnl_e9** | **str** | The realized profit and loss. | 
**position_side** | [**Side**](Side.md) |  | 
**trading_fee_e9** | **str** | The trading fee for the trade. | 
**trading_fee_asset_symbol** | **str** | The market symbol of the asset used for the trading fee. | 
**executed_at_millis** | **int** | The timestamp when the transaction was executed in milliseconds. | 
**account_address** | **str** | The address of the account. | 
**filled_quantity_e9** | **str** | The filled quantity of the order in scientific notation with 9 decimal places. | 
**side** | [**Side**](Side.md) |  | 
**leverage_e9** | **str** | The leverage applied to the position. | 
**is_isolated** | **bool** | Indicates if the position is isolated. | 
**salt** | **str** | A unique salt for the order. | 
**expires_at_millis** | **int** | The expiration timestamp of the order in milliseconds. | 
**signed_at_millis** | **int** | The signing timestamp of the order in milliseconds. | 
**reduce_only** | **bool** | Indicates if the order is reduce-only. | 
**post_only** | **bool** | Indicates if the order is post-only. | 
**time_in_force** | [**OrderTimeInForce1**](OrderTimeInForce1.md) |  | 
**trigger_price_e9** | **str** | The trigger price for stop-limit or stop-market orders. | [optional] 
**status** | [**OrderStatus1**](OrderStatus1.md) |  | 
**self_trade_prevention_type** | [**SelfTradePreventionType1**](SelfTradePreventionType1.md) |  | 
**created_at_millis** | **int** | The timestamp of the order creation in milliseconds. | 
**cancellation_reason** | [**OrderCancelReason**](OrderCancelReason.md) |  | 
**failure_to_cancel_reason** | [**OrderCancellationFailureReason**](OrderCancellationFailureReason.md) |  | [optional] 
**remaining_quantity_e9** | **str** | The remaining quantity of the order. | 
**transaction_type** | [**TransactionType**](TransactionType.md) |  | 
**amount_e9** | **str** | The amount of the transaction in scientific notation with 9 decimal places. | 
**asset_symbol** | **str** | The symbol of the asset. | [optional] 
**avg_entry_price_e9** | **str** | The average entry price for the position. | 
**liquidation_price_e9** | **str** | The liquidation price of the position. | 
**mark_price_e9** | **str** | The current mark price of the position. | 
**notional_value_e9** | **str** | The notional value of the position. | 
**max_notional_value_e9** | **str** | The maximum notional value for the position. | 
**size_e9** | **str** | The size of the position. | 
**unrealized_pnl_e9** | **str** | The unrealized profit and loss for the position. | 
**initial_margin_e9** | **str** | The initial margin required for the position. | 
**maintenance_margin_e9** | **str** | The maintenance margin required for the position. | 
**isolated_margin_e9** | **str** | The isolated margin applied to the position. | 

## Example

```python
from openapi_client.models.account_stream_message_payload import AccountStreamMessagePayload

# TODO update the JSON string below
json = "{}"
# create an instance of AccountStreamMessagePayload from a JSON string
account_stream_message_payload_instance = AccountStreamMessagePayload.from_json(json)
# print the JSON string representation of the object
print(AccountStreamMessagePayload.to_json())

# convert the object into a dict
account_stream_message_payload_dict = account_stream_message_payload_instance.to_dict()
# create an instance of AccountStreamMessagePayload from a dict
account_stream_message_payload_from_dict = AccountStreamMessagePayload.from_dict(account_stream_message_payload_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


