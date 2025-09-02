# AccountStreamMessagePayload

The payload of the message, which varies based on the event type.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_id** | **str** | The optional group ID of the account. | [optional] 
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
**updated_at_millis** | **int** | The last update time for the position in milliseconds. | 
**assets** | [**List[Asset]**](Asset.md) |  | 
**authorized_accounts** | **List[str]** | Deprecated: Replaced with authorizedWallets. | 
**authorized_wallets** | [**List[AuthorizedWallet]**](AuthorizedWallet.md) | The wallets that are authorized to trade on behalf of the current account. | 
**trade** | [**Trade**](Trade.md) |  | 
**order_hash** | **str** | The unique hash of the order. | 
**client_order_id** | **str** | The client-provided order ID. | [optional] 
**symbol** | **str** | The symbol of the market. | 
**account_address** | **str** | The address of the account. | 
**price_e9** | **str** | The price of the order in scientific notation with 9 decimal places. | 
**quantity_e9** | **str** | The quantity of the order in scientific notation with 9 decimal places. | 
**filled_quantity_e9** | **str** | The filled quantity of the order in scientific notation with 9 decimal places. | 
**side** | [**PositionSide**](PositionSide.md) |  | 
**leverage_e9** | **str** | The leverage of the order in scientific notation with 9 decimal places. | 
**is_isolated** | **bool** | Indicates if the position is isolated. | 
**salt** | **str** | A unique salt for the order. | 
**expires_at_millis** | **int** | The expiration timestamp of the order in milliseconds. | 
**signed_at_millis** | **int** | The signing timestamp of the order in milliseconds. | 
**signer_address** | **str** | The address of the signer of the order request. | 
**type** | [**OrderType**](OrderType.md) |  | 
**reduce_only** | **bool** | Indicates if the order is reduce-only. | 
**post_only** | **bool** | Indicates if the order is post-only. | 
**time_in_force** | [**OrderTimeInForce**](OrderTimeInForce.md) |  | [default to OrderTimeInForce.GTT]
**trigger_price_e9** | **str** | The trigger price for stop-limit or stop-market orders. | [optional] 
**status** | [**OrderStatus**](OrderStatus.md) |  | 
**self_trade_prevention_type** | [**SelfTradePreventionType**](SelfTradePreventionType.md) |  | [default to SelfTradePreventionType.MAKER]
**created_at_millis** | **int** | The timestamp of the order creation in milliseconds. | 
**cancellation_reason** | [**OrderCancelReason**](OrderCancelReason.md) |  | 
**failure_to_cancel_reason** | [**OrderCancellationFailureReason**](OrderCancellationFailureReason.md) |  | [optional] 
**remaining_quantity_e9** | **str** | The remaining quantity of the order. | 
**transaction_type** | [**TransactionType**](TransactionType.md) |  | 
**amount_e9** | **str** | The amount of the transaction in scientific notation with 9 decimal places. | 
**asset_symbol** | **str** | The symbol of the asset. | [optional] 
**trade_id** | **str** | The trade ID associated with the transaction. | [optional] 
**executed_at_millis** | **int** | The timestamp when the transaction was executed in milliseconds. | 
**avg_entry_price_e9** | **str** | The average entry price for the position. | 
**client_set_leverage_e9** | **str** | The leverage applied to the position. | 
**liquidation_price_e9** | **str** | The liquidation price of the position. | 
**mark_price_e9** | **str** | The current mark price of the position. | 
**notional_value_e9** | **str** | The notional value of the position. | 
**size_e9** | **str** | The size of the position. | 
**unrealized_pnl_e9** | **str** | The unrealized profit and loss for the position. | 
**margin_required_e9** | **str** | The margin required for the position. | 
**maintenance_margin_e9** | **str** | The maintenance margin required for the position. | 
**isolated_margin_e9** | **str** | The isolated margin applied to the position. | 
**reason** | **str** | The reason for the failure. | 
**reason_code** | [**CommandFailureReasonCode**](CommandFailureReasonCode.md) |  | [optional] 
**failed_command_type** | **str** | The type of command that failed. | 
**failed_command_type_code** | [**FailedCommandType**](FailedCommandType.md) |  | [optional] 
**failed_at_millis** | **int** | The timestamp when the command failed in milliseconds. | 

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


