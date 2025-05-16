# AccountOrderUpdate

A message containing order update information.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_hash** | **str** | The unique hash of the order. | 
**client_order_id** | **str** | The client-provided order ID. | [optional] 
**symbol** | **str** | The symbol of the market. | 
**account_address** | **str** | The address of the account. | 
**price_e9** | **str** | The price of the order in scientific notation with 9 decimal places. | 
**quantity_e9** | **str** | The quantity of the order in scientific notation with 9 decimal places. | 
**filled_quantity_e9** | **str** | The filled quantity of the order in scientific notation with 9 decimal places. | 
**side** | [**TradeSide**](TradeSide.md) |  | 
**leverage_e9** | **str** | The leverage of the order in scientific notation with 9 decimal places. | 
**is_isolated** | **bool** | Indicates if the order is isolated. | 
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
**updated_at_millis** | **int** | The timestamp of the last update of the order in milliseconds. | 
**cancellation_reason** | [**OrderCancelReason**](OrderCancelReason.md) |  | 
**failure_to_cancel_reason** | [**OrderCancellationFailureReason**](OrderCancellationFailureReason.md) |  | [optional] 
**remaining_quantity_e9** | **str** | The remaining quantity of the order. | 

## Example

```python
from openapi_client.models.account_order_update import AccountOrderUpdate

# TODO update the JSON string below
json = "{}"
# create an instance of AccountOrderUpdate from a JSON string
account_order_update_instance = AccountOrderUpdate.from_json(json)
# print the JSON string representation of the object
print(AccountOrderUpdate.to_json())

# convert the object into a dict
account_order_update_dict = account_order_update_instance.to_dict()
# create an instance of AccountOrderUpdate from a dict
account_order_update_from_dict = AccountOrderUpdate.from_dict(account_order_update_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


